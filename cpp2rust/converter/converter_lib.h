#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/DeclCXX.h>
#include <clang/AST/Expr.h>
#include <clang/AST/Mangle.h>
#include <clang/AST/StmtCXX.h>
#include <clang/AST/Type.h>
#include <llvm/ADT/APSInt.h>
#include <llvm/ADT/STLFunctionalExtras.h>

#include <optional>
#include <string>
#include <string_view>
#include <tuple>
#include <unordered_set>
#include <vector>

#include "logging.h"

namespace clang {
class Sema;
} // namespace clang

namespace cpp2rust {

bool IsBuiltinConstantP(const clang::Expr *expr);

bool IsGlobalVar(const clang::VarDecl *decl);

bool IsGlobalVar(const clang::Expr *expr);

bool IsRustConstEvaluableExpr(const clang::Expr *expr);

std::string GetEscapedCharLiteral(char character);

std::string GetAssertMessageAsString(const clang::Expr *expr,
                                     const clang::ASTContext &ctx);

bool IsComparisonWithNullOp(const clang::BinaryOperator *expr);

bool IsInMainFile(const clang::Decl *decl);

bool IsUnionArrayMember(const clang::Expr *base);

clang::Expr *SubscriptedArray(clang::Expr *base);

bool IsStringLiteralExpr(const clang::Expr *expr);

bool IsCodeUnitStringLiteral(const clang::StringLiteral *expr);

bool IsUserDefinedDecl(const clang::Decl *decl);

bool IsAbstractUserClass(const clang::RecordDecl *decl);

bool ExprHasUserDefinedDecl(const clang::Expr *expr);

bool IsUnsignedArithOp(const clang::BinaryOperator *expr);

llvm::APSInt GetIntegerLiteralValue(const clang::ASTContext &ctx,
                                    const clang::IntegerLiteral *expr,
                                    const clang::QualType *type,
                                    bool char_is_signed);

bool IsMut(clang::QualType qual_type);

bool RustSizeDivergesFromC(clang::QualType qt);

bool IsMutatingCall(const clang::CallExpr *expr);

// The method of a prefix or postfix increment or decrement, or nullptr if
// opcode is neither.
const char *GetIncDecMethod(clang::UnaryOperatorKind opcode);

// Calls fn on the methods of decl and on the instantiations of its method
// templates.
void ForEachMethod(const clang::CXXRecordDecl *decl,
                   llvm::function_ref<void(clang::CXXMethodDecl *)> fn);

void ForEachTemplateInstantiatedMethod(
    const clang::CXXRecordDecl *decl,
    llvm::function_ref<void(clang::CXXMethodDecl *)> fn);

bool IsOverloadedMethod(const clang::CXXMethodDecl *decl);

unsigned GetMethodIndex(const clang::CXXMethodDecl *decl);

const char *GetCopyOrMoveName(const clang::CXXMethodDecl *method);

bool CanUseCopyOrMoveName(const clang::CXXMethodDecl *decl,
                          const std::string &name);

bool IsUserDefinedCopyConstructor(const clang::CXXConstructorDecl *ctor);

bool IsConvertibleCopyOrMoveConstructor(const clang::CXXConstructorDecl *ctor);

bool IsDefaultedMoveConstructor(const clang::CXXConstructorDecl *ctor);

bool IsConvertibleMoveConstructor(const clang::CXXConstructorDecl *ctor);

// Whether decl has a move constructor converted to a move_from method.
bool HasMoveFromConstructor(const clang::RecordDecl *decl);

void ForEachConvertingConstructor(
    clang::CXXRecordDecl *decl,
    llvm::function_ref<void(clang::CXXConstructorDecl *)> fn);

bool IsImplicitConversionOperator(const clang::CXXMethodDecl *method);

bool HasImplicitConversions(const clang::RecordDecl *decl);

bool IsConvertibleMoveAssignment(const clang::CXXMethodDecl *method);

bool IsConvertibleCopyAssignment(const clang::CXXMethodDecl *method);

// Whether decl has a copy assignment operator converted to a copy_assign
// method.
bool HasCopyAssignOperator(const clang::RecordDecl *decl);

// Whether decl has a move assignment operator converted to a move_assign
// method.
bool HasMoveAssignOperator(const clang::RecordDecl *decl);

bool IsConvertibleImplicitMember(const clang::CXXMethodDecl *method);

clang::CXXConstructorDecl *
GetUserDefinedCopyConstructor(const clang::RecordDecl *decl);

bool HasCallableCopyConstructor(const clang::RecordDecl *decl);

bool HasDefaultedCopyConstructor(const clang::RecordDecl *decl);

bool RecordHasOnlyReferenceFields(const clang::RecordDecl *decl);

bool HasDefaultedCopyAssignment(const clang::RecordDecl *decl);

bool IsRValueConvertingConstructor(const clang::CXXConstructorDecl *ctor);

bool IsPassThroughConstructor(const clang::CXXConstructorDecl *ctor);

bool MethodNeedsMutableReceiver(const clang::CXXMethodDecl *method);

bool IsConvertibleCXXRecordDecl(const clang::CXXRecordDecl *decl);

bool IsConvertibleCXXMethodDecl(const clang::CXXMethodDecl *decl);

bool IsComparisonOperator(const clang::FunctionDecl *fn);

void DefineImplicitMembers(clang::Sema &sema, clang::CXXRecordDecl *decl);

bool IsEmittableMethod(clang::CXXMethodDecl *method);

bool IsMethodOnPtr(const clang::CXXMethodDecl *method);

bool IsConvertibleFunctionDecl(const clang::FunctionDecl *decl);

bool IsStdDecl(const clang::NamedDecl *decl, llvm::StringRef name);

bool IsUniquePtr(clang::QualType type);

bool IsCallToOstream(clang::CallExpr *expr);

bool IsCDecl(const clang::NamedDecl *decl, llvm::StringRef name);

bool IsStdFunction(const clang::Expr *expr, llvm::StringRef name);

bool IsAsciiStringLiteral(const clang::StringLiteral *str);

bool IsInitExprOfStringLiteral(const clang::InitListExpr *expr);

bool IsLiteral(const clang::Expr *expr);

std::vector<clang::CXXConstructorDecl *>
GetTemplateInstantiatedCtors(clang::CXXRecordDecl *decl);

unsigned GetNumberOfConvertingCtors(clang::CXXRecordDecl *decl);

unsigned GetCtorIndex(clang::CXXConstructorDecl *ctor);

clang::CXXConstructorDecl *
GetUserDefinedDefaultConstructor(const clang::CXXRecordDecl *decl);

bool HasUsableDefaultArg(const clang::ParmVarDecl *param);

const clang::MaterializeTemporaryExpr *
GetDefaultArgTemporary(const clang::ParmVarDecl *param);

std::string GetFileName(const clang::Decl *decl);

unsigned GetLineNumber(const clang::Decl *decl);

unsigned GetColumnNumber(const clang::Decl *decl);

unsigned GetArraySize(clang::QualType array_type);

void SetMangleContext(clang::MangleContext *context);

clang::MangleContext *GetMangleContext();

std::string GetID(const clang::Decl *decl);
std::string GetMethodID(const clang::CXXMethodDecl *decl);

void EscapeRustKeyword(std::string &name);

std::string GetNamedDeclAsString(const clang::NamedDecl *decl);

std::string DisambiguateAnonymousTag(const clang::TagDecl *tag);

clang::QualType GetTypeForDecl(clang::ASTContext &ctx,
                               const clang::NamedDecl *decl);

bool HasFunctionParameterPack(const clang::FunctionDecl *decl);

const char *AccessSpecifierAsString(clang::AccessSpecifier spec);

template <class T> llvm::SmallString<16> GetNumAsString(const T &num) {
  llvm::SmallString<16> small_string;
  num.toString(small_string, 10, false);
  return small_string;
}

std::string GetNumAsString(llvm::APFloat value);

clang::QualType GetReturnTypeOfFunction(const clang::CallExpr *expr);

const char *GetOverloadedOperator(const clang::FunctionDecl *decl);

std::string GetFunctionBaseName(const clang::FunctionDecl *decl);

void ToIdentifier(std::string &name);

// Whether record has a method with the given identifier.
bool HasMethodNamed(const clang::CXXRecordDecl *record, llvm::StringRef name);

bool IsImplicitAssignmentCall(const clang::CallExpr *expr);
bool IsUserOperatorCall(const clang::CXXOperatorCallExpr *expr);

const clang::CXXRecordDecl *AsLambdaClass(clang::QualType type);

const clang::CXXMethodDecl *AsLambdaOperatorCall(const clang::FunctionDecl *fn);

void ForEachLambdaCallOperator(
    const clang::CXXRecordDecl *decl,
    llvm::function_ref<void(clang::CXXMethodDecl *)> fn);

clang::CXXMethodDecl *
AsLambdaCallOperator(const clang::CXXConversionDecl *conversion);

unsigned GetLambdaSpecializationIndex(const clang::FunctionDecl *call_operator);

const clang::LambdaCapture *AsLambdaCapture(const clang::FieldDecl *field);

bool IsLambdaCopyOrMoveConstructor(const clang::CXXConstructorDecl *ctor);

bool LambdaNeedsDestruction(const clang::CXXRecordDecl *decl);

clang::Expr *AsLambdaUncapturedConstant(const clang::FunctionDecl *fn,
                                        clang::DeclRefExpr *expr);

const clang::FieldDecl *AsLambdaCaptureThis(const clang::FunctionDecl *fn);

clang::QualType GetDeclRefType(const clang::FunctionDecl *fn,
                               const clang::Expr *expr,
                               const clang::ValueDecl *decl);

bool IsSameTypeComparison(const clang::FunctionDecl *fn,
                          const clang::CXXRecordDecl *record);

clang::CXXDestructorDecl *
GetUserDefinedDestructor(const clang::CXXRecordDecl *decl);

bool TypeNeedsDestruction(clang::QualType type);

bool HasFieldsNeedingDestruction(const clang::CXXRecordDecl *decl);

bool RecordNeedsDestruction(const clang::CXXRecordDecl *decl);

// The fields of decl that need destruction, in the order they are destroyed.
std::vector<const clang::FieldDecl *>
FieldsToDestroy(const clang::CXXRecordDecl *decl);

// The parameters of the definition of decl if there is one, as their names
// may differ from those of the declaration.
llvm::ArrayRef<clang::ParmVarDecl *>
DefinitionParameters(const clang::FunctionDecl *decl);

clang::Expr *ToAddrOf(clang::ASTContext &ctx, clang::Expr *expr);

clang::ConditionalOperator *
MakeConditionalAddrOf(clang::ASTContext &ctx, clang::ConditionalOperator *expr);

clang::CXXConstructExpr *MakeConstructExpr(clang::ASTContext &ctx,
                                           clang::QualType type,
                                           clang::CXXConstructorDecl *ctor,
                                           llvm::ArrayRef<clang::Expr *> args);

std::optional<clang::ArrayRef<clang::TemplateArgument>>
GetTemplateArgs(clang::QualType qual_type, clang::Expr *expr = nullptr);

clang::Expr *GetCallObject(clang::CallExpr *expr);

clang::Expr *GetCallee(clang::CallExpr *expr);

std::unordered_set<const clang::ValueDecl *>
GetAllVars(const clang::Stmt *stmt);

bool ReferencesVar(const clang::Stmt *stmt, const clang::VarDecl *var);

// Whether a and b refer to a common variable or field.
bool ReferToCommonVar(const clang::Stmt *a, const clang::Stmt *b);

// Whether stmt refers to a pointer or reference variable or field, through
// which it may access any object.
bool RefersToPointer(const clang::Stmt *stmt);

bool DefaultInitHasSideEffects(const clang::ASTContext &ctx,
                               clang::QualType type);

bool ReferencesThis(const clang::Stmt *stmt);

// Whether evaluating `stmt` reads a value from memory, e.g., a pointer.
bool ReadsMemory(const clang::Stmt *stmt);

// Whether stmt materializes a temporary, which a value may point to.
bool CreatesTemporary(const clang::Stmt *stmt);

bool MayCauseBorrowMutError(const clang::Expr *lhs, const clang::Expr *rhs);

bool ArgsMayAlias(const clang::Expr *a, const clang::Expr *b);

bool HasReceiver(clang::Expr *expr);

std::optional<clang::QualType> GetParamType(clang::Expr *expr,
                                            unsigned arg_idx);

std::optional<clang::QualType> GetParamImplicitConvertTarget(clang::Expr *expr,
                                                             unsigned arg_idx);

// Build unified args for a call expression: for member calls, the receiver
// becomes a0 and call args shift to a1, a2, etc. For operator/free calls,
// args are used as-is.
std::vector<clang::Expr *>
BuildUnifiedArgs(clang::Expr *expr, clang::Expr **args, unsigned num_args);

clang::Expr *BuildInitExpr(clang::Sema &sema, clang::QualType type,
                           llvm::ArrayRef<clang::Expr *> args,
                           clang::SourceLocation loc);

bool IsVaListType(clang::QualType type);

bool IsAliasOfInit(clang::ASTContext &ctx, const clang::VarDecl *var);

bool NeedsImplicitScalarCast(clang::ASTContext &ctx, clang::QualType from,
                             clang::QualType to);

clang::QualType GetExprPointee(clang::ASTContext &ctx, const clang::Expr *from,
                               clang::QualType to);

bool NeedsImplicitPointeeCast(clang::ASTContext &ctx, const clang::Expr *from,
                              clang::QualType to);

bool NeedsRefBindingTemp(clang::ASTContext &ctx, const clang::Expr *arg,
                         clang::QualType param_type);

// The type of the reference parameter that argument i of call is bound to
// through a temporary, if it is.
std::optional<clang::QualType>
GetRefBindingTempType(clang::ASTContext &ctx, const clang::CallExpr *call,
                      unsigned i);

// The value that a transparent union argument is initialized with, or expr.
clang::Expr *IgnoreTransparentUnion(clang::Expr *expr);

bool IsSizeType(clang::ASTContext &ctx, clang::QualType type);

std::optional<clang::QualType> GetOperandImplicitConversionTarget(
    clang::ASTContext &ctx, const clang::BinaryOperator *op,
    const clang::Expr *operand, const clang::Expr *sibling);

bool IsBuiltinVaStart(const clang::CallExpr *expr);

bool IsBuiltinVaEnd(const clang::CallExpr *expr);

bool IsBuiltinVaCopy(const clang::CallExpr *expr);

bool IsTransparentStdCall(const clang::CallExpr *expr);

const clang::Expr *IgnoreTransparentStdCall(const clang::Expr *expr);

bool IsTemporaryObject(const clang::Expr *expr);

bool ContainsVAArgExpr(const clang::Stmt *stmt);

clang::Expr *NormalizeToBool(clang::Expr *expr, clang::ASTContext &ctx);

struct SwitchArm {
  std::vector<clang::Stmt *> body;
  llvm::StringRef label;
  clang::SwitchCase *head;
  bool is_default_case;
  bool has_fallthrough;
};

std::vector<SwitchArm> AnalyzeSwitchArms(clang::CompoundStmt *body);

bool CompoundHasTopLevelLabel(const clang::CompoundStmt *compound);

std::string_view Trim(std::string_view s);

void Unwrap(std::string &s, std::string_view prefix, std::string_view suffix);

void ReplaceAll(std::string &str, std::string_view from, std::string_view to);

// The method that casts a pointer to from into a pointer to to when only
// their constness differs, or nullptr if it doesn't.
const char *GetConstCastSuffix(clang::QualType to, clang::QualType from);

} // namespace cpp2rust
