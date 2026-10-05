// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/models/boxed_vars.h"

#include <clang/AST/RecursiveASTVisitor.h>

#include "converter/converter_lib.h"

namespace cpp2rust {
namespace {
// Whether decl may be stored without a Value, if its address is not taken:
// scalars and arrays of non-arrays.
bool CanUnbox(const clang::VarDecl *decl) {
  auto type = decl->getType();
  if (type->isConstantArrayType()) {
    if (type->getAsArrayTypeUnsafe()->getElementType()->isArrayType()) {
      return false;
    }
  } else if (!type->isScalarType()) {
    return false;
  }
  return (decl->isLocalVarDecl() || clang::isa<clang::ParmVarDecl>(decl)) &&
         !IsGlobalVar(decl) && !decl->isInitCapture() &&
         !IsVaListType(decl->getType());
}

// Statements are visited before their children, so the uses of a variable
// that only access its value are recorded before the variable is reached.
class AddressTakenVisitor
    : public clang::RecursiveASTVisitor<AddressTakenVisitor> {
public:
  explicit AddressTakenVisitor(std::unordered_set<const clang::VarDecl *> &vars)
      : address_taken_(vars) {}

  bool shouldVisitTemplateInstantiations() const { return true; }

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

  bool VisitBinaryOperator(clang::BinaryOperator *expr) {
    if (expr->isAssignmentOp()) {
      AddValueUse(expr->getLHS());
    }
    return true;
  }

  bool VisitUnaryOperator(clang::UnaryOperator *expr) {
    if (expr->isIncrementDecrementOp()) {
      AddValueUse(expr->getSubExpr());
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
  // An element of an array is accessed without making a pointer to it.
  void AddValueUse(clang::Expr *expr) {
    expr = expr->IgnoreParens();
    if (auto *subscript = clang::dyn_cast<clang::ArraySubscriptExpr>(expr)) {
      auto *cast =
          clang::dyn_cast<clang::ImplicitCastExpr>(subscript->getBase());
      if (cast && cast->getCastKind() == clang::CK_ArrayToPointerDecay) {
        AddValueUse(cast->getSubExpr());
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

  std::unordered_set<const clang::VarDecl *> &address_taken_;
  // The references to variables that only access their value.
  std::unordered_set<const clang::DeclRefExpr *> value_uses_;
};
} // namespace

BoxedVars::BoxedVars(clang::ASTContext &ctx) {
  AddressTakenVisitor(address_taken_)
      .TraverseDecl(ctx.getTranslationUnitDecl());
}

bool BoxedVars::contains(const clang::VarDecl *decl) const {
  return !CanUnbox(decl) || address_taken_.contains(decl);
}
} // namespace cpp2rust
