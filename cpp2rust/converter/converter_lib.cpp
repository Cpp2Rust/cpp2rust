// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/converter_lib.h"

#include <clang/AST/ExprCXX.h>
#include <clang/AST/Mangle.h>
#include <clang/AST/ParentMapContext.h>
#include <clang/Basic/SourceManager.h>

#include <array>
#include <filesystem>
#include <unordered_set>

#include "converter/lex.h"
#include "converter/mapper.h"

// https://doc.rust-lang.org/reference/keywords.html
static const char *rust_keywords[] = {
    // Strict keywords
    "as",
    "async",
    "await",
    "crate",
    "dyn",
    "fn",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "self",
    "Self",
    "super",
    "trait",
    "type",
    "unsafe",
    "use",
    "where",
    // Reserved keywords
    "abstract",
    "become",
    "box",
    "final",
    "gen",
    "macro",
    "override",
    "priv",
    "unsized",
    "yield",
    // Weak keywords
    "macro_rules",
    "raw",
    "safe",
    // Standard library keywords
    "vec",
};

namespace cpp2rust {

bool IsGlobalVar(clang::VarDecl *decl) {
  return decl->isFileVarDecl() || decl->isStaticLocal();
}

bool IsGlobalVar(clang::Expr *expr) {
  expr = expr->IgnoreImplicit();
  clang::DeclRefExpr *decl_ref = clang::dyn_cast<clang::DeclRefExpr>(expr);
  if (!decl_ref) {
    return false;
  }

  if (auto *decl = clang::dyn_cast<clang::VarDecl>(decl_ref->getDecl())) {
    return IsGlobalVar(decl);
  }
  return false;
}

const clang::ValueDecl *GetBuiltinDecl(const clang::Expr *expr) {
  if (clang::isa<clang::ImplicitCastExpr>(expr)) {
    const auto *implicit_cast = clang::cast<clang::ImplicitCastExpr>(expr);
    const auto *sub_expr = implicit_cast->getSubExpr();
    if (implicit_cast->getCastKind() == clang::CK_BuiltinFnToFnPtr &&
        clang::isa<clang::DeclRefExpr>(sub_expr)) {
      const auto *decl_ref = clang::cast<clang::DeclRefExpr>(sub_expr);
      const auto *value_decl = decl_ref->getDecl();
      return value_decl->getAsFunction();
    }
  }
  return nullptr;
}

bool IsBuiltinConstantP(const clang::Expr *expr) {
  if (const auto *value_decl = GetBuiltinDecl(expr); value_decl) {
    return value_decl->getName() == "__builtin_constant_p";
  }
  return false;
}

bool IsComparisonWithNullOp(const clang::BinaryOperator *expr) {
  const auto *rhs = expr->getRHS()->IgnoreCasts();
  switch (rhs->getStmtClass()) {
  case clang::Stmt::CXXNullPtrLiteralExprClass:
    return expr->isComparisonOp();
  default:
    return false;
  }
}

bool IsInMainFile(const clang::Decl *decl) {
  const auto &ctx = decl->getASTContext();
  const auto &src_mgr = ctx.getSourceManager();
  const auto loc = decl->getBeginLoc();
  return src_mgr.isInMainFile(src_mgr.getExpansionLoc(loc));
}

bool IsConvertibleDecl(const clang::Decl *decl) {
  const auto &ctx = decl->getASTContext();
  const auto &src_mgr = ctx.getSourceManager();
  const auto src_loc = decl->getLocation();
  return !decl->getBeginLoc().isInvalid() && !decl->isImplicit() &&
         !src_mgr.isInSystemHeader(src_loc) &&
         !src_mgr.isInSystemMacro(src_loc);
}

bool IsUnsignedArithOp(const clang::BinaryOperator *expr) {
  clang::QualType lhs_type;
  clang::QualType rhs_type;
  bool add_or_mul_op = false;
  if (expr->isCompoundAssignmentOp()) {
    const auto *compound_assign_op =
        clang::cast<clang::CompoundAssignOperator>(expr);
    lhs_type = compound_assign_op->getComputationLHSType();
    rhs_type = compound_assign_op->getComputationResultType();
    add_or_mul_op = expr->getOpcode() <= clang::BO_SubAssign;
  } else {
    lhs_type = expr->getLHS()->getType();
    rhs_type = expr->getRHS()->getType();
    add_or_mul_op = expr->isAdditiveOp() || expr->isMultiplicativeOp();
  }
  return !expr->getType()->isPointerType() && add_or_mul_op &&
         (lhs_type->isUnsignedIntegerType() ||
          rhs_type->isUnsignedIntegerType());
}

bool IsMut(clang::QualType qual_type) {
  return !qual_type.isConstQualified() &&
         !(qual_type->isReferenceType() &&
           qual_type->getPointeeType().isConstQualified());
}

bool IsOverloadedFunction(const clang::FunctionDecl *decl) {
  const auto *ctx = decl->getDeclContext();
  const auto decl_name = decl->getDeclName();
  const auto lookup_result = ctx->lookup(decl_name);
  return !lookup_result.isSingleResult();
}

bool IsOverloadedMethod(const clang::CXXMethodDecl *decl) {
  const auto method_name = decl->getNameAsString();
  const auto *record = decl->getParent();
  return std::count_if(record->method_begin(), record->method_end(),
                       [&method_name](const auto *method) {
                         return method->getNameAsString() == method_name;
                       }) > 1;
}

bool IsConvertibleCXXRecordDecl(const clang::CXXRecordDecl *decl) {
  return decl->isThisDeclarationADefinition() &&
         std::all_of(
             decl->method_begin(), decl->method_end(), [](auto *method) {
               return method->getDefinition() || method->isPureVirtual();
             });
}

bool IsConvertibleCXXMethodDecl(const clang::CXXMethodDecl *decl) {
  // Destructors go into the Drop trait
  return !llvm::isa<clang::CXXDestructorDecl>(decl) && !decl->isImplicit();
}

bool IsConvertibleFunctionDecl(const clang::FunctionDecl *decl) {
  return decl->hasBody() && decl->isThisDeclarationADefinition();
}

bool IsUniquePtr(clang::QualType type) {
  auto *record_decl = type.getNonReferenceType()->getAsCXXRecordDecl();
  return record_decl && record_decl->getNameAsString() == "unique_ptr";
}

bool IsCallToOstream(clang::CallExpr *expr) {
  if (auto *op = clang::dyn_cast<clang::CXXOperatorCallExpr>(expr)) {
    if (auto *record_decl = expr->getType()->getAsCXXRecordDecl()) {
      return record_decl->getNameAsString() == "basic_ostream" &&
             op->getOperator() == clang::OverloadedOperatorKind::OO_LessLess;
    }
  }
  return false;
}

std::vector<clang::CXXConstructorDecl *>
GetTemplateInstantiatedCtors(clang::CXXRecordDecl *decl) {
  std::vector<clang::CXXConstructorDecl *> out;
  for (auto d : decl->decls()) {
    if (auto function_template_decl =
            llvm::dyn_cast<clang::FunctionTemplateDecl>(d)) {
      for (auto s : function_template_decl->specializations()) {
        if (auto c = clang::dyn_cast<clang::CXXConstructorDecl>(s)) {
          if (c->getDefinition()) {
            out.push_back(c);
          }
        }
      }
    }
  }
  return out;
}

unsigned GetNumberOfConvertingCtors(clang::CXXRecordDecl *decl) {
  return llvm::count_if(decl->ctors(),
                        [](const clang::CXXConstructorDecl *c) {
                          return !c->isCopyOrMoveConstructor() &&
                                 !c->isImplicit();
                        }) +
         GetTemplateInstantiatedCtors(decl).size();
}

unsigned GetCtorIndex(clang::CXXConstructorDecl *ctor) {
  static std::unordered_map<std::string, std::vector<std::string>>
      index_per_record;
  auto &vec = index_per_record[GetID(ctor->getParent())];
  auto id = GetID(ctor);

  auto it = std::find(vec.begin(), vec.end(), id);
  if (it == vec.end()) {
    vec.push_back(id);
    return vec.size();
  }
  return static_cast<unsigned>(std::distance(vec.begin(), it) + 1);
}

clang::CXXConstructorDecl *
GetUserDefinedDefaultConstructor(const clang::CXXRecordDecl *decl) {
  for (auto c : decl->ctors()) {
    if (c->isUserProvided() && c->isDefaultConstructor()) {
      return c;
    }
  }
  return nullptr;
}

std::string GetMainFileName(const clang::ASTContext &ctx) {
  const auto &src_mgr = ctx.getSourceManager();
  auto filed_id = src_mgr.getMainFileID();
  auto file_entry = src_mgr.getFileEntryRefForID(filed_id);
  auto file_path = file_entry->getName();
  auto file_name = llvm::sys::path::filename(file_path);
  return llvm::sys::path::stem(file_name).str();
}

std::string GetFileName(const clang::Decl *decl) {
  const auto &ctx = decl->getASTContext();
  const auto full_location = ctx.getFullLoc(decl->getBeginLoc());
  const auto file_name = ctx.getSourceManager().getFilename(full_location);
  const auto file_name_as_string = file_name.str();
  const std::filesystem::path file_path(file_name_as_string);
  return std::filesystem::exists(file_path)
             ? std::filesystem::canonical(file_path).string()
             : file_name_as_string;
}

unsigned GetLineNumber(const clang::Decl *decl) {
  const auto &ctx = decl->getASTContext();
  const auto &src_mgr = ctx.getSourceManager();
  const auto loc = decl->getLocation();
  return src_mgr.getPresumedLineNumber(loc);
}

unsigned GetColumnNumber(const clang::Decl *decl) {
  const auto &ctx = decl->getASTContext();
  const auto &src_mgr = ctx.getSourceManager();
  const auto loc = decl->getLocation();
  return src_mgr.getPresumedColumnNumber(loc);
}

unsigned GetArraySize(clang::QualType array_type) {
  assert(array_type->isArrayType());
  auto constant_array_ty = clang::dyn_cast<clang::ConstantArrayType>(
      array_type->getAsArrayTypeUnsafe());
  assert(constant_array_ty);
  return constant_array_ty->getSize().getZExtValue();
}

std::string GetID(const clang::Decl *decl) {
  assert(decl);
  const auto file_name = GetFileName(decl);
  const auto line_num = GetLineNumber(decl);
  const auto column_num = GetColumnNumber(decl);
  std::string args;
  if (auto fdecl = clang::dyn_cast<clang::FunctionDecl>(decl)) {
    for (unsigned i = 0; i < fdecl->getNumParams(); ++i) {
      args += fdecl->getParamDecl(i)->getType().getAsString();
    }
  }
  return file_name + std::to_string(line_num) + std::to_string(column_num) +
         args;
}

static std::unordered_map<std::string, size_t> type_mapping;

std::string GetNamedDeclAsString(const clang::NamedDecl *decl) {
  auto name = decl->getDeclName().isIdentifier() ? decl->getName().str()
                                                 : decl->getNameAsString();

  if (auto *fn = clang::dyn_cast<clang::FunctionDecl>(decl)) {
    if (!clang::isa<clang::CXXMethodDecl>(fn)) {
      auto mangled =
          clang::ASTNameGenerator(decl->getASTContext()).getName(decl) +
          GetID(decl);
      auto id =
          type_mapping.try_emplace(mangled, type_mapping.size()).first->second;
      name += '_';
      name += std::to_string(id);
    }
  }

  // transform decl names that are rust keywords:
  // keyword -> keyword_
  // keyword_ -> keyword__
  // etc
  for (auto &keyword : rust_keywords) {
    if (!name.starts_with(keyword))
      continue;

    auto suffix = std::string_view(name).substr(strlen(keyword));
    if (std::ranges::all_of(suffix, [](char c) { return c == '_'; })) {
      name += '_';
      break;
    }
  }

  if (name.size() == 0) {
    name = "self";
  }

  return name;
}

const char *AccessSpecifierAsString(clang::AccessSpecifier spec) {
  switch (spec) {
  case clang::AS_public:
  case clang::AS_none:
    return keyword::kPub;
  case clang::AS_protected:
  case clang::AS_private:
    return token::kEmpty;
  }
  std::unreachable();
}

clang::QualType GetReturnTypeOfFunction(const clang::CallExpr *expr) {
  if (auto *decl = expr->getCalleeDecl()) {
    if (auto *fn = decl->getAsFunction()) {
      return fn->getReturnType().getCanonicalType();
    }
  }

  auto callee_ty = expr->getCallee()->getType();
  if (auto *ptr_ty = callee_ty->getAs<clang::PointerType>()) {
    if (auto *fn_ty =
            ptr_ty->getPointeeType()->getAs<clang::FunctionProtoType>()) {
      return fn_ty->getReturnType().getCanonicalType();
    }
  }

  assert(0 && "Unhandled function prototype");
  return {};
}

const char *GetOverloadedOperator(const clang::FunctionDecl *decl) {
  switch (decl->getOverloadedOperator()) {
  case clang::OO_Less:
    return "lt";
  default:
    // FIXME: improve error handling
    llvm::errs() << "unsupported overloaded operator\n";
    return "";
  }
}

bool IsOverloadedComparisonOperator(const clang::CXXMethodDecl *decl) {
  if (decl->isOverloadedOperator()) {
    switch (decl->getOverloadedOperator()) {
    case clang::OO_EqualEqual:
    case clang::OO_ExclaimEqual:
    case clang::OO_Less:
    case clang::OO_Greater:
    case clang::OO_LessEqual:
    case clang::OO_GreaterEqual:
    case clang::OO_Spaceship:
      return true;
    default:
      return false;
    }
  }
  return false;
}

clang::Expr *ToAddrOf(clang::ASTContext &ctx, clang::Expr *expr) {
  return clang::UnaryOperator::Create(
      ctx, expr, clang::UnaryOperatorKind::UO_AddrOf, expr->getType(),
      expr->getValueKind(), expr->getObjectKind(), expr->getExprLoc(), false,
      {});
}

std::vector<clang::CXXRecordDecl *>
GetNestedStructs(const clang::CXXRecordDecl *decl) {
  std::vector<clang::Decl *> nested_decls;
  auto predicate = [](auto *decl) {
    return clang::isa<clang::CXXRecordDecl>(decl) && !decl->isImplicit();
  };
  std::copy_if(decl->decls_begin(), decl->decls_end(),
               std::back_inserter(nested_decls), predicate);
  std::vector<clang::CXXRecordDecl *> nested_record_decls;
  auto op = [](auto *decl) { return clang::cast<clang::CXXRecordDecl>(decl); };
  std::transform(nested_decls.cbegin(), nested_decls.cend(),
                 std::back_inserter(nested_record_decls), op);
  return nested_record_decls;
}

std::optional<clang::ArrayRef<clang::TemplateArgument>>
GetTemplateArgs(clang::QualType qual_type, clang::Expr *expr) {
  if (auto ty = clang::dyn_cast<clang::TemplateSpecializationType>(qual_type)) {
    return ty->template_arguments();
  }

  if (auto *decl = qual_type->getAsCXXRecordDecl()) {
    if (auto *tpldecl =
            clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(decl)) {
      return tpldecl->getTemplateArgs().asArray();
    }
  }

  if (!expr) {
    return std::nullopt;
  }

  if (auto *call = clang::dyn_cast<clang::CallExpr>(expr)) {
    expr = call->getCallee();
  }
  expr = expr->IgnoreCasts();

  switch (expr->getStmtClass()) {
  case clang::Stmt::MemberExprClass: {
    if (auto *member = clang::dyn_cast<clang::MemberExpr>(expr)) {
      if (auto *method =
              clang::dyn_cast<clang::CXXMethodDecl>(member->getMemberDecl())) {
        qual_type = method->getThisType();
      }
    }
    break;
  }
  case clang::Stmt::DeclRefExprClass: {
    if (auto *value_decl =
            clang::dyn_cast<clang::DeclRefExpr>(expr)->getDecl()) {
      if (auto *method = clang::dyn_cast<clang::CXXMethodDecl>(value_decl);
          method && method->isOverloadedOperator()) {
        qual_type = method->getThisType();
        break;
      } else if (auto *function = value_decl->getAsFunction()) {
        qual_type = function->getReturnType();
        break;
      }
    }
    break;
  }
  default:
    qual_type = expr->getType()->getCanonicalTypeInternal();
    break;
  }

  if (auto pt = qual_type->getPointeeType(); !pt.isNull())
    qual_type = pt;

  return GetTemplateArgs(qual_type);
}

clang::Expr *GetCallObject(clang::CallExpr *expr) {
  if (auto *member = clang::dyn_cast<clang::MemberExpr>(
          expr->getCallee()->IgnoreParenImpCasts())) {
    return member->getBase();
  }
  if (auto *opcall = clang::dyn_cast<clang::CXXOperatorCallExpr>(expr)) {
    if (opcall->getNumArgs() > 0)
      return opcall->getArg(0)->IgnoreParenImpCasts();
  }
  return nullptr;
}

std::vector<clang::Expr *>
BuildUnifiedArgs(clang::Expr *expr, clang::Expr **args, unsigned num_args) {
  std::vector<clang::Expr *> all_args;
  if (auto *mcall = clang::dyn_cast<clang::CXXMemberCallExpr>(expr)) {
    all_args.push_back(mcall->getImplicitObjectArgument());
  } else if (auto *member = clang::dyn_cast<clang::MemberExpr>(expr)) {
    all_args.push_back(member->getBase());
  }
  for (unsigned i = 0; i < num_args; ++i) {
    all_args.push_back(args[i]);
  }
  return all_args;
}

clang::Expr *GetCalleeOrExpr(clang::Expr *expr) {
  if (auto *call = clang::dyn_cast<clang::CallExpr>(expr)) {
    return call->getCallee();
  }
  return expr;
}

bool HasReceiver(clang::Expr *expr) {
  if (clang::isa<clang::CXXMemberCallExpr>(expr))
    return true;
  if (auto *opcall = clang::dyn_cast<clang::CXXOperatorCallExpr>(expr)) {
    if (auto *callee = opcall->getCalleeDecl()) {
      return clang::isa<clang::CXXMethodDecl>(callee);
    }
  }
  return false;
}

std::optional<IteratorCategory>
GetStrongestIteratorCategory(clang::QualType type) {
  type = type.getNonReferenceType().getUnqualifiedType();
  if (!Mapper::Contains(type)) {
    return std::nullopt;
  }
  if (Mapper::MapsToRefcountPointer(type)) {
    return IteratorCategory::Contiguous;
  }
  auto mapped = Mapper::Map(type);
  if (mapped.empty()) {
    return std::nullopt;
  }
  if (mapped.starts_with("RefcountMapIter<") ||
      mapped.starts_with("UnsafeMapIterator<")) {
    return IteratorCategory::Bidirectional;
  }
  return std::nullopt;
}

const clang::CXXForRangeStmt *
GetParentForRange(clang::ASTContext &ctx, const clang::MemberExpr *member) {
  auto base = member->getBase()->IgnoreParenImpCasts();
  if (auto decl_ref = llvm::dyn_cast<clang::DeclRefExpr>(base)) {
    if (auto var = llvm::dyn_cast<clang::VarDecl>(decl_ref->getDecl())) {
      for (const auto &parent : ctx.getParents(*var)) {
        if (auto decl_stmt = parent.get<clang::DeclStmt>()) {
          for (const auto &grandparent : ctx.getParents(*decl_stmt)) {
            if (auto for_range = grandparent.get<clang::CXXForRangeStmt>()) {
              return for_range;
            }
          }
        }
      }
    }
  }
  return nullptr;
}

clang::QualType
GetForRangeIteratorType(const clang::CXXForRangeStmt *for_range) {
  if (auto begin_stmt = for_range->getBeginStmt()) {
    if (auto begin_decl_stmt = llvm::dyn_cast<clang::DeclStmt>(begin_stmt)) {
      if (auto begin_var = llvm::dyn_cast<clang::VarDecl>(
              begin_decl_stmt->getSingleDecl())) {
        return begin_var->getType();
      }
    }
  }
  return {};
}

std::string GetClassName(clang::QualType type) {
  if (auto *record = type->getAsCXXRecordDecl())
    return record->getQualifiedNameAsString();
  return {};
}

bool IsRedundantCopyInConversion(clang::ASTContext &ctx,
                                 const clang::CXXConstructExpr *expr) {
  auto parents = ctx.getParentMapContext().getParents(*expr);
  if (parents.empty()) {
    return false;
  }
  auto *parent = parents[0].get<clang::CXXConstructExpr>();
  return parent && parent->getConstructor()->isConvertingConstructor(false);
}

bool IsVaListType(clang::QualType type) {
  for (auto t = type; !t.isNull();) {
    if (auto *adjusted = t->getAs<clang::AdjustedType>()) {
      // Possibly decayed va_list
      t = adjusted->getOriginalType();
      continue;
    } else if (auto *typedef_type = t->getAs<clang::TypedefType>()) {
      // Typedef'ed va_list
      if (auto decl = typedef_type->getDecl()) {
        if (decl->getName().contains("va_list")) {
          return true;
        }
        t = decl->getUnderlyingType();
        continue;
      }
    }
    break;
  }
  return false;
}

bool IsBuiltinVaStart(const clang::CallExpr *expr) {
  if (auto *fn = expr->getDirectCallee()) {
    return fn->getBuiltinID() == clang::Builtin::BI__builtin_va_start;
  }
  return false;
}

bool IsBuiltinVaEnd(const clang::CallExpr *expr) {
  if (auto *fn = expr->getDirectCallee()) {
    return fn->getBuiltinID() == clang::Builtin::BI__builtin_va_end;
  }
  return false;
}

bool IsBuiltinVaCopy(const clang::CallExpr *expr) {
  if (auto *fn = expr->getDirectCallee()) {
    return fn->getBuiltinID() == clang::Builtin::BI__builtin_va_copy;
  }
  return false;
}

bool ContainsVAArgExpr(const clang::Stmt *stmt) {
  if (clang::isa<clang::VAArgExpr>(stmt)) {
    return true;
  }
  for (auto *child : stmt->children()) {
    if (ContainsVAArgExpr(child)) {
      return true;
    }
  }
  return false;
}

clang::Expr *CreateConversionToBool(clang::Expr *expr, clang::ASTContext &ctx) {
  return clang::ImplicitCastExpr::Create(
      ctx, ctx.BoolTy, clang::CK_IntegralToBoolean, expr,
      /*BasePath=*/nullptr, clang::VK_PRValue, clang::FPOptionsOverride());
}

std::vector<clang::SwitchCase *>
GetTopLevelSwitchCases(clang::SwitchStmt *stmt) {
  std::vector<clang::SwitchCase *> cases;
  if (auto *body = llvm::dyn_cast<clang::CompoundStmt>(stmt->getBody())) {
    for (auto *s : body->body()) {
      if (auto *sc = clang::dyn_cast<clang::SwitchCase>(s)) {
        cases.push_back(sc);
      }
    }
  }
  return cases;
}

bool SwitchCaseContainsDefault(clang::SwitchCase *c) {
  for (clang::Stmt *cur = c;;) {
    if (clang::isa<clang::DefaultStmt>(cur)) {
      return true;
    }
    auto *sc = clang::dyn_cast<clang::SwitchCase>(cur);
    if (!sc) {
      return false;
    }
    cur = sc->getSubStmt();
  }
  return false;
}

static clang::Stmt *GetLastStmtOfSwitchCase(clang::SwitchCase *c) {
  clang::Stmt *cur = c->getSubStmt();
  while (auto *sc = clang::dyn_cast<clang::SwitchCase>(cur)) {
    cur = sc->getSubStmt();
  }
  return cur;
}

std::vector<clang::Stmt *> GetSwitchCaseBody(clang::CompoundStmt *body,
                                             clang::SwitchCase *head) {
  std::vector<clang::Stmt *> out;
  out.push_back(GetLastStmtOfSwitchCase(head));
  auto it = body->body_begin(), end = body->body_end();
  while (it != end && *it != head) {
    ++it;
  }
  assert(it != end);
  ++it;
  while (it != end && !clang::isa<clang::SwitchCase>(*it)) {
    out.push_back(*it);
    ++it;
  }
  return out;
}

static bool SwitchCaseHasFallthrough(clang::Stmt *stmt) {
  if (!stmt) {
    return false;
  }
  if (auto *compound = clang::dyn_cast<clang::CompoundStmt>(stmt)) {
    if (compound->body_empty()) {
      return true;
    }
    return SwitchCaseHasFallthrough(compound->body_back());
  }
  if (clang::isa<clang::BreakStmt>(stmt) ||
      clang::isa<clang::ReturnStmt>(stmt)) {
    return false;
  }
  return true;
}

bool SwitchHasFallthrough(clang::SwitchStmt *stmt) {
  if (auto *body = clang::dyn_cast<clang::CompoundStmt>(stmt->getBody())) {
    for (auto top_level_case : GetTopLevelSwitchCases(stmt)) {
      auto arm = GetSwitchCaseBody(body, top_level_case);
      if (arm.empty() || SwitchCaseHasFallthrough(arm.back())) {
        return true;
      }
    }
  }
  return false;
}

} // namespace cpp2rust
