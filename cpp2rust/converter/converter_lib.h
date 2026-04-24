#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/Expr.h>
#include <clang/AST/StmtCXX.h>
#include <clang/AST/Type.h>

#include <optional>
#include <regex>
#include <string>
#include <vector>

namespace cpp2rust {

// Order matters: each category is a superset of the previous one.
// Use >= to check "at least this capable".
enum class IteratorCategory {
  Forward,
  Bidirectional,
  RandomAccess,
  Contiguous,
};

std::optional<IteratorCategory>
GetStrongestIteratorCategory(clang::QualType type);
bool IsBuiltinConstantP(const clang::Expr *expr);

bool IsGlobalVar(clang::VarDecl *decl);

bool IsGlobalVar(clang::Expr *expr);

bool IsComparisonWithNullOp(const clang::BinaryOperator *expr);

bool IsInMainFile(const clang::Decl *decl);

bool IsConvertibleDecl(const clang::Decl *decl);

bool IsUnsignedArithOp(const clang::BinaryOperator *expr);

bool IsMut(clang::QualType qual_type);

bool IsOverloadedFunction(const clang::FunctionDecl *decl);

bool IsOverloadedMethod(const clang::CXXMethodDecl *decl);

bool IsConvertibleCXXRecordDecl(const clang::CXXRecordDecl *decl);

bool IsConvertibleCXXMethodDecl(const clang::CXXMethodDecl *decl);

bool IsConvertibleFunctionDecl(const clang::FunctionDecl *decl);

bool IsUniquePtr(clang::QualType type);

bool IsCallToOstream(clang::CallExpr *expr);

std::vector<clang::CXXConstructorDecl *>
GetTemplateInstantiatedCtors(clang::CXXRecordDecl *decl);

unsigned GetNumberOfConvertingCtors(clang::CXXRecordDecl *decl);

unsigned GetCtorIndex(clang::CXXConstructorDecl *ctor);

clang::CXXConstructorDecl *
GetUserDefinedDefaultConstructor(const clang::CXXRecordDecl *decl);

std::string GetMainFileName(const clang::ASTContext &ctx);

std::string GetFileName(const clang::Decl *decl);

unsigned GetLineNumber(const clang::Decl *decl);

unsigned GetColumnNumber(const clang::Decl *decl);

unsigned GetArraySize(clang::QualType array_type);

std::string GetID(const clang::Decl *decl);

std::string GetNamedDeclAsString(const clang::NamedDecl *decl);

const char *AccessSpecifierAsString(clang::AccessSpecifier spec);

template <class T> llvm::SmallString<16> GetNumAsString(const T &num) {
  llvm::SmallString<16> small_string;
  num.toString(small_string, 10, false);
  return small_string;
}

clang::QualType GetReturnTypeOfFunction(const clang::CallExpr *expr);

const char *GetOverloadedOperator(const clang::FunctionDecl *decl);

bool IsOverloadedComparisonOperator(const clang::CXXMethodDecl *decl);

clang::Expr *ToAddrOf(clang::ASTContext &ctx, clang::Expr *expr);

std::vector<clang::CXXRecordDecl *>
GetNestedStructs(const clang::CXXRecordDecl *decl);

std::optional<clang::ArrayRef<clang::TemplateArgument>>
GetTemplateArgs(clang::QualType qual_type, clang::Expr *expr = nullptr);

template <class UnaryFunction>
void ForEachTemplateArgument(
    clang::ArrayRef<clang::TemplateArgument> template_arguments,
    UnaryFunction unary_function) {
  for (auto template_argument : template_arguments) {
    switch (template_argument.getKind()) {
    case clang::TemplateArgument::ArgKind::Type:
      unary_function(template_argument.getAsType());
      break;
    default:
      // FIXME: improve logging
      llvm::errs() << "unsupported template argument kind\n";
    }
  }
}

clang::Expr *GetCallObject(clang::CallExpr *expr);

clang::Expr *GetCalleeOrExpr(clang::Expr *expr);

bool HasReceiver(clang::Expr *expr);

// Build unified args for a call expression: for member calls, the receiver
// becomes a0 and call args shift to a1, a2, etc. For operator/free calls,
// args are used as-is.
std::vector<clang::Expr *>
BuildUnifiedArgs(clang::Expr *expr, clang::Expr **args, unsigned num_args);

const clang::CXXForRangeStmt *
GetParentForRange(clang::ASTContext &ctx, const clang::MemberExpr *member);

clang::QualType
GetForRangeIteratorType(const clang::CXXForRangeStmt *for_range);

std::string GetClassName(clang::QualType type);

bool IsRedundantCopyInConversion(clang::ASTContext &ctx,
                                 const clang::CXXConstructExpr *expr);

bool IsVaListType(clang::QualType type);

bool IsBuiltinVaStart(const clang::CallExpr *expr);

bool IsBuiltinVaEnd(const clang::CallExpr *expr);

bool IsBuiltinVaCopy(const clang::CallExpr *expr);

bool ContainsVAArgExpr(const clang::Stmt *stmt);

clang::Expr *CreateConversionToBool(clang::Expr *expr, clang::ASTContext &ctx);

std::vector<clang::SwitchCase *>
GetTopLevelSwitchCases(clang::SwitchStmt *stmt);

bool SwitchCaseContainsDefault(clang::SwitchCase *c);

std::vector<clang::Stmt *> GetSwitchCaseBody(clang::CompoundStmt *body,
                                             clang::SwitchCase *head);

bool SwitchHasFallthrough(clang::SwitchStmt *stmt);

} // namespace cpp2rust
