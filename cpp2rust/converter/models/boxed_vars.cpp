// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/models/boxed_vars.h"

#include <clang/AST/RecursiveASTVisitor.h>

#include <algorithm>
#include <unordered_map>
#include <vector>

#include "converter/converter_lib.h"
#include "converter/mapper.h"
#include "converter/rules/registry.h"

namespace cpp2rust {
namespace {
// Whether type is translated to a Vec whose elements are not in Values:
// std::vector and std::string of scalars or user-defined structs.
bool IsFlatVec(clang::QualType type) {
  auto *record =
      clang::dyn_cast_or_null<clang::ClassTemplateSpecializationDecl>(
          type->getAsCXXRecordDecl());
  if (!record || !record->isInStdNamespace() ||
      (record->getName() != "vector" && record->getName() != "basic_string")) {
    return false;
  }
  auto element = record->getTemplateArgs()[0].getAsType();
  return element->isScalarType() ||
         (element->isStructureOrClassType() &&
          IsUserDefinedDecl(element->getAsRecordDecl()));
}

// Whether decl may be stored without a Value, if its address is not taken:
// scalars, arrays of non-arrays, user-defined structs, unique_ptrs and
// vectors, unless they have a destructor, which is called with a pointer to
// them.
bool CanUnbox(const clang::VarDecl *decl) {
  auto type = decl->getType();
  if (IsFlatVec(type)) {
    // Vectors are flat Vecs.
  } else if (type->isConstantArrayType()) {
    if (type->getAsArrayTypeUnsafe()->getElementType()->isArrayType()) {
      return false;
    }
  } else if (type->isStructureOrClassType()) {
    if ((!IsUserDefinedDecl(type->getAsRecordDecl()) && !IsUniquePtr(type)) ||
        TypeNeedsDestruction(type)) {
      return false;
    }
  } else if (!type->isScalarType()) {
    return false;
  }
  return (decl->isLocalVarDecl() || clang::isa<clang::ParmVarDecl>(decl)) &&
         !IsGlobalVar(decl) && !decl->isInitCapture() &&
         !IsVaListType(decl->getType());
}

clang::Expr *IgnoreNoOpCasts(clang::Expr *expr) {
  while (auto *cast = clang::dyn_cast<clang::ImplicitCastExpr>(expr)) {
    if (cast->getCastKind() != clang::CK_NoOp) {
      break;
    }
    expr = cast->getSubExpr();
  }
  return expr;
}

// The object copied by a trivial copy or move constructor, which copies its
// value, like an lvalue-to-rvalue conversion in C.
clang::Expr *GetTrivialCopySource(clang::CXXConstructExpr *expr) {
  auto *ctor = expr->getConstructor();
  if (!ctor->isCopyOrMoveConstructor() || !ctor->isTrivial()) {
    return nullptr;
  }
  return IgnoreNoOpCasts(expr->getArg(0));
}

// Whether expr is a call to a trivial copy or move assignment operator, which
// assigns the value of its right operand, like a built-in assignment, unless
// it is translated to a call to the operator.
bool IsTrivialAssignment(clang::CXXOperatorCallExpr *expr) {
  auto *method =
      clang::dyn_cast_or_null<clang::CXXMethodDecl>(expr->getDirectCallee());
  return method &&
         (method->isCopyAssignmentOperator() ||
          method->isMoveAssignmentOperator()) &&
         method->isTrivial() && !IsUserOperatorCall(expr);
}

// Statements are visited before their children, so the uses of a variable
// that only access its value are recorded before the variable is reached.
class AddressTakenVisitor
    : public clang::RecursiveASTVisitor<AddressTakenVisitor> {
public:
  AddressTakenVisitor(clang::ASTContext &ctx,
                      std::unordered_set<const clang::VarDecl *> &vars)
      : ctx_(ctx), address_taken_(vars) {}

  bool shouldVisitTemplateInstantiations() const { return true; }

  // Only the semantic form of an initializer list is converted, in which the
  // values of variables are read through lvalue-to-rvalue conversions.
  bool TraverseInitListExpr(clang::InitListExpr *expr,
                            DataRecursionQueue *queue = nullptr) {
    return TraverseSynOrSemInitListExpr(
        expr->isSemanticForm() ? expr : expr->getSemanticForm(), queue);
  }

  bool VisitDeclRefExpr(clang::DeclRefExpr *expr) {
    if (!value_uses_.contains(expr)) {
      AddressTaken(expr->getDecl());
    }
    return true;
  }

  bool VisitImplicitCastExpr(clang::ImplicitCastExpr *expr) {
    if (expr->getCastKind() == clang::CK_LValueToRValue) {
      AddValueUse(expr->getSubExpr());
    }
    return true;
  }

  bool VisitExplicitCastExpr(clang::ExplicitCastExpr *expr) {
    if (expr->getCastKind() == clang::CK_ToVoid) {
      AddValueUse(expr->getSubExpr());
    }
    return true;
  }

  bool VisitCXXConstructExpr(clang::CXXConstructExpr *expr) {
    if (auto *source = GetTrivialCopySource(expr)) {
      AddValueUse(source);
    }
    AddRuleArgs(expr, expr->getArgs(), expr->getNumArgs());
    return true;
  }

  bool VisitCallExpr(clang::CallExpr *expr) {
    AddRuleArgs(expr, expr->getArgs(), expr->getNumArgs());
    return true;
  }

  bool VisitCXXOperatorCallExpr(clang::CXXOperatorCallExpr *expr) {
    if (IsTrivialAssignment(expr)) {
      AddValueUse(expr->getArg(0), /*write=*/true);
      AddValueUse(IgnoreNoOpCasts(expr->getArg(1)));
    }
    // Dereferencing a unique_ptr reads the pointer it holds; the object it
    // points to is not stored in the unique_ptr.
    switch (expr->getOperator()) {
    case clang::OO_Subscript:
    case clang::OO_Star:
    case clang::OO_Arrow:
      if (IsUniquePtr(expr->getArg(0)->getType())) {
        AddValueUse(IgnoreNoOpCasts(expr->getArg(0)));
      }
      break;
    default:
      break;
    }
    return true;
  }

  bool VisitBinaryOperator(clang::BinaryOperator *expr) {
    if (expr->isAssignmentOp()) {
      AddValueUse(expr->getLHS(), /*write=*/true);
    }
    return true;
  }

  bool VisitUnaryOperator(clang::UnaryOperator *expr) {
    if (expr->isIncrementDecrementOp()) {
      AddValueUse(expr->getSubExpr(), /*write=*/true);
    }
    return true;
  }

  bool VisitUnaryExprOrTypeTraitExpr(clang::UnaryExprOrTypeTraitExpr *expr) {
    if (!expr->isArgumentType()) {
      AddValueUse(expr->getArgumentExpr());
    }
    return true;
  }

  bool VisitLambdaExpr(clang::LambdaExpr *expr) {
    for (const auto &capture : expr->captures()) {
      if (capture.capturesVariable()) {
        AddressTaken(capture.getCapturedVar());
      }
    }
    return true;
  }

private:
  // The arguments of a call translated by a rule are passed by value or
  // borrowed, unless the rule takes a pointer to them.
  void AddRuleArgs(clang::Expr *expr, clang::Expr **args, unsigned num_args) {
    auto *rule = RuleRegistry::GetExprRule(ctx_, expr);
    if (!rule) {
      return;
    }
    // Variadic arguments are not considered.
    auto all_args = BuildUnifiedArgs(expr, args, num_args);
    for (unsigned i = 0; i < all_args.size() && i < rule->params.size(); ++i) {
      if (!rule->params[i].is_pointer()) {
        AddValueUse(IgnoreNoOpCasts(all_args[i]));
      }
    }
  }

  // An element of an array or vector, or a field of a struct, is accessed
  // without making a pointer to it.
  void AddValueUse(clang::Expr *expr, bool write = false) {
    expr = expr->IgnoreParens();
    if (auto *member = clang::dyn_cast<clang::MemberExpr>(expr)) {
      auto *field = clang::dyn_cast<clang::FieldDecl>(member->getMemberDecl());
      // A union member is accessed through a pointer to the union.
      if (!member->isArrow() && field && !field->getParent()->isUnion()) {
        AddValueUse(member->getBase(), write);
      }
    } else if (auto *subscript =
                   clang::dyn_cast<clang::ArraySubscriptExpr>(expr)) {
      auto *cast =
          clang::dyn_cast<clang::ImplicitCastExpr>(subscript->getBase());
      if (cast && cast->getCastKind() == clang::CK_ArrayToPointerDecay) {
        AddValueUse(cast->getSubExpr(), write);
      }
    } else if (auto *op = clang::dyn_cast<clang::CXXOperatorCallExpr>(expr)) {
      // Rust doesn't let the index of an element that is written to read the
      // vector, as the vector is borrowed mutably first.
      auto *vec = IgnoreNoOpCasts(op->getArg(0));
      if (op->getOperator() == clang::OO_Subscript &&
          IsFlatVec(vec->getType()) &&
          (!write || std::ranges::none_of(GetAllVars(vec), [&](auto *var) {
            return GetAllVars(op->getArg(1)).contains(var);
          }))) {
        AddValueUse(vec, write);
      }
    } else if (auto *ref = clang::dyn_cast<clang::DeclRefExpr>(expr)) {
      value_uses_.insert(ref);
    }
  }

  void AddressTaken(const clang::ValueDecl *decl) {
    if (auto *var = clang::dyn_cast<clang::VarDecl>(decl);
        var && CanUnbox(var)) {
      address_taken_.insert(var);
    }
  }

  clang::ASTContext &ctx_;
  std::unordered_set<const clang::VarDecl *> &address_taken_;
  // The references to variables that only access their value.
  std::unordered_set<const clang::DeclRefExpr *> value_uses_;
};
} // namespace

BoxedVars::BoxedVars(clang::ASTContext &ctx) {
  AddressTakenVisitor(ctx, address_taken_)
      .TraverseDecl(ctx.getTranslationUnitDecl());
}

bool BoxedVars::contains(const clang::VarDecl *decl) const {
  return !CanUnbox(decl) || address_taken_.contains(decl);
}

namespace {
class MovableReadsVisitor
    : public clang::RecursiveASTVisitor<MovableReadsVisitor> {
public:
  explicit MovableReadsVisitor(clang::ASTContext &ctx) : ctx_(ctx) {}

  bool shouldVisitTemplateInstantiations() const { return true; }

  std::unordered_set<const clang::DeclRefExpr *> Find(clang::Expr *expr) {
    TraverseStmt(expr);
    std::unordered_set<const clang::DeclRefExpr *> movable;
    for (auto *read : reads_) {
      if (refs_[read->getDecl()] == 1) {
        movable.insert(read);
      }
    }
    return movable;
  }

  bool dataTraverseStmtPre(clang::Stmt *stmt) {
    excluded_ += IsExcluded(stmt);
    return true;
  }

  bool dataTraverseStmtPost(clang::Stmt *stmt) {
    excluded_ -= IsExcluded(stmt);
    return true;
  }

  bool VisitDeclRefExpr(clang::DeclRefExpr *expr) {
    ++refs_[expr->getDecl()];
    return true;
  }

  bool VisitImplicitCastExpr(clang::ImplicitCastExpr *expr) {
    if (expr->getCastKind() == clang::CK_LValueToRValue) {
      AddRead(expr->getSubExpr());
    }
    return true;
  }

  bool VisitCXXConstructExpr(clang::CXXConstructExpr *expr) {
    if (auto *source = GetTrivialCopySource(expr)) {
      AddRead(source);
    }
    return true;
  }

private:
  void AddRead(clang::Expr *expr) {
    if (auto *ref = clang::dyn_cast<clang::DeclRefExpr>(expr->IgnoreParens());
        ref && excluded_ == 0) {
      reads_.push_back(ref);
    }
  }

  // Whether the reads in stmt may be translated more than once, or evaluated
  // more than once, e.g., in a loop or in a closure.
  bool IsExcluded(clang::Stmt *stmt) const {
    if (auto *call = clang::dyn_cast<clang::CallExpr>(stmt)) {
      return Mapper::Contains(ctx_, call->getCallee());
    }
    if (auto *construct = clang::dyn_cast<clang::CXXConstructExpr>(stmt)) {
      return !GetTrivialCopySource(construct);
    }
    return clang::isa<clang::CXXNewExpr, clang::StmtExpr, clang::LambdaExpr,
                      clang::BinaryConditionalOperator>(stmt);
  }

  clang::ASTContext &ctx_;
  unsigned excluded_ = 0;
  std::unordered_map<const clang::ValueDecl *, unsigned> refs_;
  std::vector<const clang::DeclRefExpr *> reads_;
};
} // namespace

std::unordered_set<const clang::DeclRefExpr *>
FindMovableReads(clang::ASTContext &ctx, clang::Expr *expr) {
  return MovableReadsVisitor(ctx).Find(expr);
}
} // namespace cpp2rust
