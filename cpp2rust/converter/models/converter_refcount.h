#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/converter.h"

namespace cpp2rust {
class ConverterRefCount final : public Converter {
public:
  ConverterRefCount(std::string &rs_code, clang::ASTContext &ctx);

  std::string EmitFilePreamble() override;

  RsExpr *Convert(clang::QualType qual_type) override;

  RsExpr *
  VisitIncompleteArrayType(const clang::IncompleteArrayType *type) override;

  RsExpr *
  VisitLValueReferenceType(const clang::LValueReferenceType *type) override;

  RsExpr *VisitPointerType(const clang::PointerType *type) override;

  RsExpr *VisitRecordType(const clang::RecordType *type) override;

  RsExpr *VisitConstantArrayType(const clang::ConstantArrayType *type) override;

  RsExpr *
  ConvertFunctionPointerType(const clang::FunctionProtoType *proto,
                             FnProtoType kind = FnProtoType::FnPtr) override;

  RsExpr *VisitCXXRecordDecl(clang::CXXRecordDecl *decl) override;

  RsExpr *VisitOffsetOfExpr(clang::OffsetOfExpr *expr) override;

  RsExpr *EmitRustUnion(clang::RecordDecl *decl) override;

  bool EmitsReprCForRecords() const override { return false; }

  const char *CharRustType() const override { return "u8"; }

  RsExpr *ConvertOrdAndPartialOrdTraits(const clang::CXXRecordDecl *decl,
                                        const clang::FunctionDecl *op) override;

  RsExpr *AddCloneTrait(const clang::RecordDecl *decl) override;

  RsExpr *AddDropTrait(const clang::CXXRecordDecl *decl) override;

  RsExpr *AddByteReprTrait(const clang::RecordDecl *decl) override;

  RsExpr *AddByteReprTrait(const clang::EnumDecl *decl) override;

  RsExpr *
  VisitUnaryExprOrTypeTraitExpr(clang::UnaryExprOrTypeTraitExpr *expr) override;

  RsExpr *AddDefaultTrait(const clang::RecordDecl *decl) override;

  RsExpr *AddDefaultTraitForUnion(const clang::RecordDecl *decl) override;

  std::string GetSelfMaybeWithMut(const clang::CXXMethodDecl *decl) override;

  RsExpr *VisitCXXConstructorDecl(clang::CXXConstructorDecl *decl) override;

  RsExpr *VisitFieldDecl(clang::FieldDecl *decl) override;

  RsExpr *EmitFunctionPreamble(clang::FunctionDecl *decl) override;

  RsExpr *VisitVarDecl(clang::VarDecl *decl) override;

  RsExpr *ConvertGlobalVarDecl(clang::VarDecl *decl) override;

  RsExpr *ConvertVaListVarDecl(clang::VarDecl *decl) override;

  std::pair<RsExpr *, bool>
  ConvertVarDeclSkipInit(clang::VarDecl *decl) override;

  RsExpr *EmitHoistedInArmAssignment(clang::VarDecl *decl) override;

  bool ConvertLambdaVarDecl(clang::VarDecl *decl) override;

  RsExpr *VisitDeclRefExpr(clang::DeclRefExpr *expr) override;

  RsExpr *ConvertIncAndDec(clang::UnaryOperator *expr) override;

  RsExpr *VisitConditionalOperator(clang::ConditionalOperator *expr) override;

  RsExpr *ConvertPrintf(clang::CallExpr *expr) override;

  RsExpr *EmitFnPtrCall(clang::Expr *callee) override;

  RsExpr *
  ConvertFunctionToFunctionPointer(const clang::FunctionDecl *fn_decl) override;

  // FnPtr does not implement Copy
  bool FunctionPointerImplementsCopy() const override { return false; }

  RsExpr *VisitCallExpr(clang::CallExpr *expr) override;

  RsExpr *VisitStringLiteral(clang::StringLiteral *expr) override;

  RsExpr *VisitImplicitCastExpr(clang::ImplicitCastExpr *expr) override;

  RsExpr *VisitFunctionPointerCast(clang::ExplicitCastExpr *expr);

  RsExpr *VisitExplicitCastExpr(clang::ExplicitCastExpr *expr) override;

  RsExpr *ConvertBinaryOperator(clang::BinaryOperator *expr) override;

  RsExpr *VisitStmtExpr(clang::StmtExpr *expr) override;

  RsExpr *EmitStmtExprTail(clang::Expr *tail) override;

  RsExpr *VisitInitListExpr(clang::InitListExpr *expr) override;

  RsExpr *VisitArraySubscriptExpr(clang::ArraySubscriptExpr *expr) override;

  RsExpr *VisitMemberExpr(clang::MemberExpr *expr) override;

  RsExpr *ConvertUnionMemberAccessor(clang::MemberExpr *expr);

  RsExpr *VisitCXXNewExpr(clang::CXXNewExpr *expr) override;

  RsExpr *VisitCXXDeleteExpr(clang::CXXDeleteExpr *expr) override;

  RsExpr *VisitCXXForRangeStmtMap(clang::CXXForRangeStmt *stmt) override;

  RsExpr *VisitCXXForRangeStmtVector(clang::CXXForRangeStmt *stmt) override;

  RsExpr *VisitCXXForRangeStmtString(clang::CXXForRangeStmt *stmt) override;

  RsExpr *EmitByValueShadow(const std::string &loop_var_name,
                            clang::QualType type, RsExpr *box_expr,
                            const std::string &type_override = "");

  RsExpr *ConvertStream(clang::Expr *expr) override;

  RsExpr *VisitCXXConstructExpr(clang::CXXConstructExpr *expr) override;

  RsExpr *
  VisitImplicitValueInitExpr(clang::ImplicitValueInitExpr *expr) override;

  RsExpr *VisitVAArgExpr(clang::VAArgExpr *expr) override;

  RsExpr *ConvertVariadicArg(clang::Expr *arg) override;

  RsExpr *ConvertArrayCXXConstructExpr(clang::CXXConstructExpr *expr) override;

  RsExpr *VisitCXXDefaultArgExpr(clang::CXXDefaultArgExpr *expr) override;

  RsExpr *GetDefaultAsString(clang::QualType qual_type) override;

  RsExpr *GetArrayDefaultAsString(clang::QualType qual_type) override;

  RsExpr *ConvertEqualsNullPtr(clang::Expr *expr) override;

  RsExpr *GetDefaultAsStringFallback(clang::QualType qual_type) override;

  RsExpr *ConvertVarDefaultInit(clang::QualType qual_type) override;

  std::vector<const char *>
  GetStructAttributes(const clang::RecordDecl *decl) override;

  RsExpr *ConvertVarInit(clang::QualType qual_type, clang::Expr *expr) override;

  RsExpr *ConvertVarInitValue(clang::QualType qual_type, clang::Expr *expr);

  RsExpr *ConvertAssignment(clang::Expr *lhs, clang::Expr *rhs,
                            std::string_view assign_operator) override;

  RsExpr *ConvertUniquePtrDeref(clang::CXXOperatorCallExpr *expr) override;

  RsExpr *ConvertCXXOperatorCallExpr(clang::CXXOperatorCallExpr *expr) override;

  RsExpr *ConvertFunctionParameters(clang::FunctionDecl *decl) override;

  RsExpr *ConvertArraySubscript(clang::Expr *base, clang::Expr *idx,
                                clang::QualType type) override;
  RsExpr *ConvertPointerSubscript(clang::ArraySubscriptExpr *expr) override;

  RsExpr *
  ConvertFunctionMain(const clang::FunctionDecl *decl,
                      const std::string_view main_function_name) override;

  RsExpr *ConvertAddrOf(clang::Expr *expr,
                        clang::QualType pointer_type) override;

  RsExpr *ConvertDeref(clang::Expr *expr) override;

  RsExpr *ConvertArrow(clang::Expr *expr) override;

  RsExpr *AccessLValueObject(clang::MemberExpr *member) override;

  RsExpr *ConvertGenericBinaryOperator(clang::BinaryOperator *expr) override;

  bool IsReferenceType(const clang::Expr *expr) const override;

  RsExpr *ConvertMappedMethodCall(clang::Expr *expr,
                                  const TranslationRule::MethodCallFragment &mc,
                                  clang::Expr **args, unsigned num_args,
                                  TempMaterializationCtx *ctx) override;

private:
  std::pair<RsExpr *, RsExpr *> MaterializeTemp(const std::string &binding_name,
                                                clang::QualType param_type,
                                                clang::Expr *expr) override;

  RsExpr *
  emplace_back_plugin_construct_arg(clang::QualType elem_type,
                                    clang::CXXConstructExpr *ctor) override;
  RsExpr *emplace_back_emit_push_open(clang::CXXMemberCallExpr *call) override;
  RsExpr *emplace_back_emit_push_close(clang::CXXMemberCallExpr *call) override;

  const char *GetPointerDerefSuffix(clang::QualType pointee_type);
  const char *GetPointerDerefPrefix(clang::QualType pointee_type) override;

  RsExpr *BuildFnAdapter(const clang::FunctionDecl *src_fn,
                         const clang::FunctionProtoType *src_proto,
                         const clang::FunctionProtoType *target_proto);

  RsExpr *EmitSetOrAssign(clang::Expr *lhs, RsExpr *rhs);

  // Wraps a pointer expression with deref prefix/suffix: e.g.
  // "(*ptr.upgrade().deref())" or "(ptr.read())"
  RsExpr *DerefPtrExpr(RsExpr *ptr, clang::QualType pointee_type);

  std::string GetInnerType(clang::QualType type);

  RsExpr *ConvertFreshLValue(clang::Expr *expr);
  RsExpr *ConvertObject(clang::Expr *expr);
  RsExpr *ConvertFreshObject(clang::Expr *expr) override;
  RsExpr *ConvertFresh(clang::Expr *expr,
                       std::optional<clang::QualType> implicit_convert_to = {});
  RsExpr *ConvertFreshRValue(
      clang::Expr *expr,
      std::optional<clang::QualType> implicit_convert_to = {}) override;
  RsExpr *ConvertFreshPointer(clang::Expr *expr) override;

  RsExpr *ConvertPtrType(clang::QualType type);
  RsExpr *ConvertPointeeType(clang::QualType ptr_type) override;

  RsExpr *ConvertSubscriptIndex(clang::Expr *idx);

  std::string GetSafeTypeAsString(clang::QualType qual_type) const;

  /// The kind of conversion that should be performed.
  enum class ConversionKind : uint8_t {
    Unboxed,
    Ptr,
    FullRefCount,
  };

  static const char *ConversionKindToString(ConversionKind k) {
    switch (k) {
    case ConversionKind::Unboxed:
      return "Unboxed";
    case ConversionKind::Ptr:
      return "Ptr";
    case ConversionKind::FullRefCount:
      return "FullRefCount";
    }
    std::unreachable();
  }

  ConversionKind getConversionKind() const { return conversion_kind_.back(); }

  struct PushConversionKind {
    ConverterRefCount &c;
    bool pushed;

    PushConversionKind(ConverterRefCount &c, ConversionKind k, bool cond = true,
                       int line = __builtin_LINE())
        : c(c), pushed(cond) {
      if (pushed) {
        c.conversion_kind_.push_back(k);
      }
      log() << "[PushConversionKind:" << line << "] ";
      for (auto ck : c.conversion_kind_) {
        log() << ConversionKindToString(ck) << ", ";
      }
      log() << '\n';
    }
    ~PushConversionKind() {
      if (pushed) {
        c.conversion_kind_.pop_back();
      }
      log() << "[PopConversionKind] ";
      for (auto ck : c.conversion_kind_) {
        log() << ConversionKindToString(ck) << ", ";
      }
      log() << '\n';
    }
  };

  struct PushUnboxedIfSimple {
    ConverterRefCount &c;
    PushUnboxedIfSimple(ConverterRefCount &c, std::string_view outer_type,
                        clang::QualType inner_type);

    ~PushUnboxedIfSimple() { c.conversion_kind_.pop_back(); }
  };

  RsExpr *BoxType(RsExpr *node);
  RsExpr *BoxValue(RsExpr *node);

  std::vector<ConversionKind> conversion_kind_;

  // Set by pointer-related visit methods (ConvertDeref,
  // ConvertPointerSubscript, etc.) when converting an LValue that goes through
  // a Ptr. Contains the ptr expression node. Consumed by EmitSetOrAssign to
  // emit ptr.write(rhs), or by ConvertMappedMethodCall to emit
  // ptr.with_mut(...).
  struct PendingDeref {
    void set(RsExpr *node, clang::Expr *expr = nullptr);
    void set_unchecked(RsExpr *node, clang::Expr *expr = nullptr);
    RsExpr *take() {
      auto *result = value;
      value = nullptr;
      pointee_is_boxed = false;
      return result;
    }
    bool empty() const { return value == nullptr; }
    bool is_boxed() const { return pointee_is_boxed; }
    void assert_consumed() const {
      assert(value == nullptr && "pending_deref_ not consumed");
    }

  private:
    static bool compute_inner_boxed(clang::Expr *expr);
    RsExpr *value = nullptr;
    bool pointee_is_boxed = false;
  } pending_deref_;
};
} // namespace cpp2rust
