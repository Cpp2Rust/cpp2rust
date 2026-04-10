#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/converter.h"

namespace cpp2rust {
class ConverterRefCount final : public Converter {
public:
  ConverterRefCount(std::string &rs_code, clang::ASTContext &ctx);

  void EmitFilePreamble() override;

  bool VisitRecordType(clang::RecordType *type) override;

  bool VisitConstantArrayType(clang::ConstantArrayType *type) override;

  bool VisitIncompleteArrayType(clang::IncompleteArrayType *type) override;

  bool VisitLValueReferenceType(clang::LValueReferenceType *type) override;

  bool VisitPointerType(clang::PointerType *type) override;

  bool VisitCXXRecordDecl(clang::CXXRecordDecl *decl) override;

  void ConvertOrdAndPartialOrdTraits(const clang::CXXRecordDecl *decl,
                                     const clang::FunctionDecl *op) override;

  void AddCloneTrait(const clang::CXXRecordDecl *decl) override;

  void AddDropTrait(const clang::CXXRecordDecl *decl) override;

  void AddByteReprTrait(const clang::CXXRecordDecl *decl) override;

  void AddDefaultTrait(const clang::CXXRecordDecl *decl) override;

  std::string GetSelfMaybeWithMut(const clang::CXXMethodDecl *decl) override;

  bool VisitCXXConstructorDecl(clang::CXXConstructorDecl *decl) override;

  bool VisitFieldDecl(clang::FieldDecl *decl) override;

  void EmitFunctionPreamble(clang::FunctionDecl *decl) override;

  bool VisitVarDecl(clang::VarDecl *decl) override;

  void ConvertGlobalVarDecl(clang::VarDecl *decl) override;

  void ConvertVaListVarDecl(clang::VarDecl *decl) override;

  bool ConvertLambdaVarDecl(clang::VarDecl *decl) override;

  bool VisitDeclRefExpr(clang::DeclRefExpr *expr) override;

  bool ConvertIncAndDec(clang::UnaryOperator *expr) override;

  bool VisitConditionalOperator(clang::ConditionalOperator *expr) override;

  void ConvertPrintf(clang::CallExpr *expr) override;

  bool VisitCallExpr(clang::CallExpr *expr) override;

  bool VisitStringLiteral(clang::StringLiteral *expr) override;

  bool VisitImplicitCastExpr(clang::ImplicitCastExpr *expr) override;

  bool VisitExplicitCastExpr(clang::ExplicitCastExpr *expr) override;

  bool VisitBinaryOperator(clang::BinaryOperator *expr) override;

  bool VisitInitListExpr(clang::InitListExpr *expr) override;

  bool VisitArraySubscriptExpr(clang::ArraySubscriptExpr *expr) override;

  bool VisitMemberExpr(clang::MemberExpr *expr) override;

  bool VisitCXXNewExpr(clang::CXXNewExpr *expr) override;

  bool VisitCXXDeleteExpr(clang::CXXDeleteExpr *expr) override;

  bool VisitCXXForRangeStmtMap(clang::CXXForRangeStmt *stmt) override;

  bool VisitCXXForRangeStmtVector(clang::CXXForRangeStmt *stmt) override;

  bool VisitCXXForRangeStmtString(clang::CXXForRangeStmt *stmt) override;

  void EmitByValueShadow(const std::string &loop_var_name, clang::QualType type,
                         std::string box_expr,
                         const std::string &type_override = "");

  std::string ConvertStream(clang::Expr *expr) override;

  bool VisitCXXConstructExpr(clang::CXXConstructExpr *expr) override;

  bool VisitImplicitValueInitExpr(clang::ImplicitValueInitExpr *expr) override;

  bool VisitVAArgExpr(clang::VAArgExpr *expr) override;

  void ConvertArrayCXXConstructExpr(clang::CXXConstructExpr *expr) override;

  bool VisitCXXDefaultArgExpr(clang::CXXDefaultArgExpr *expr) override;

  std::string GetDefaultAsString(clang::QualType qual_type) override;

  std::string GetDefaultAsStringFallback(clang::QualType qual_type) override;

  std::string ConvertVarDefaultInit(clang::QualType qual_type) override;

  std::vector<const char *>
  GetStructAttributes(const clang::CXXRecordDecl *decl,
                      bool &out_impl_default) override;

  bool MayCauseBorrowMutError(const clang::Expr *lhs, const clang::Expr *rhs);

  bool Convert(clang::QualType qual_type) override;
  bool Convert(clang::Expr *expr) override { return Converter::Convert(expr); }
  bool Convert(clang::Stmt *stmt) override {
    auto result = Converter::Convert(stmt);
    pending_deref_.assert_consumed();
    return result;
  }

  void ConvertVarInit(clang::QualType qual_type, clang::Expr *expr) override;

  void ConvertAssignment(clang::Expr *lhs, clang::Expr *rhs,
                         std::string_view assign_operator) override;

  void ConvertUniquePtrDeref(clang::CXXOperatorCallExpr *expr) override;

  bool ConvertCXXOperatorCallExpr(clang::CXXOperatorCallExpr *expr) override;

  void ConvertFunctionParameters(clang::FunctionDecl *decl) override;

  void ConvertArraySubscript(clang::Expr *base, clang::Expr *idx,
                             clang::QualType type) override;
  void ConvertPointerSubscript(clang::ArraySubscriptExpr *expr) override;

  void ConvertFunctionMain(const clang::FunctionDecl *decl,
                           const std::string_view main_function_name) override;

  void ConvertAddrOf(clang::Expr *expr, clang::QualType pointer_type) override;

  void ConvertDeref(clang::Expr *expr) override;

  void ConvertArrow(clang::Expr *expr) override;

  std::string AccessLValueObject(clang::MemberExpr *member) override;

  void ConvertGenericBinaryOperator(clang::BinaryOperator *expr) override;

  bool IsReferenceType(const clang::Expr *expr) const override;

  std::string
  ConvertMappedMethodCall(clang::Expr *expr,
                          const TranslationRule::MethodCallFragment &mc,
                          clang::Expr **args, unsigned num_args,
                          TempMaterializationCtx *ctx) override;

private:
  std::pair<std::string, std::string>
  MaterializeTemp(const std::string &binding_name, clang::QualType param_type,
                  clang::Expr *expr) override;

  void
  emplace_back_plugin_construct_arg(clang::QualType elem_type,
                                    clang::CXXConstructExpr *ctor) override;
  void emplace_back_emit_push_open(clang::CXXMemberCallExpr *call) override;
  void emplace_back_emit_push_close(clang::CXXMemberCallExpr *call) override;

  const char *GetPointerDerefSuffix(clang::QualType pointee_type);
  const char *GetPointerDerefPrefix(clang::QualType pointee_type) override;

  void EmitSetOrAssign(clang::Expr *lhs, std::string_view rhs);

  // Wraps a pointer expression with deref prefix/suffix: e.g.
  // "(*ptr.upgrade().deref())" or "(ptr.read())"
  std::string DerefPtrExpr(std::string_view ptr_expr,
                           clang::QualType pointee_type);

  std::string GetInnerType(clang::QualType type);

  std::string ConvertFreshLValue(clang::Expr *expr);
  std::string ConvertObject(clang::Expr *expr);
  std::string ConvertFreshObject(clang::Expr *expr) override;
  std::string ConvertFresh(clang::Expr *expr);
  std::string ConvertFreshRValue(clang::Expr *expr) override;
  std::string ConvertFreshPointer(clang::Expr *expr) override;

  std::string ConvertPtrType(clang::QualType type);

  /// The kind of conversion that should be performed.
  enum class ConversionKind : uint8_t {
    Unboxed,
    Ptr,
    FullRefCount,
  };

  std::string ConversionKindToString(ConversionKind k) {
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
      llvm::errs() << "[PushConversionKind:" << line << "] ";
      for (auto ck : c.conversion_kind_) {
        llvm::errs() << c.ConversionKindToString(ck) << ", ";
      }
      llvm::errs() << "\n";
    }
    ~PushConversionKind() {
      if (pushed) {
        c.conversion_kind_.pop_back();
      }
      llvm::errs() << "[PopConversionKind] ";
      for (auto ck : c.conversion_kind_) {
        llvm::errs() << c.ConversionKindToString(ck) << ", ";
      }
      llvm::errs() << "\n";
    }
  };

  struct PushUnboxedIfSimple {
    ConverterRefCount &c;
    PushUnboxedIfSimple(ConverterRefCount &c, std::string_view outer_type,
                        clang::QualType inner_type);

    ~PushUnboxedIfSimple() { c.conversion_kind_.pop_back(); }
  };

  std::string BoxType(std::string &&str) const;
  std::string BoxValue(std::string &&str) const;

  std::vector<ConversionKind> conversion_kind_;

  // Set by pointer-related visit methods (ConvertDeref,
  // ConvertPointerSubscript, etc.) when converting an LValue that goes through
  // a Ptr. Contains the ptr expression string. Consumed by EmitSetOrAssign to
  // emit ptr.write(rhs), or by ConvertMappedMethodCall to emit
  // ptr.with_mut(...).
  struct PendingDeref {
    void set(std::string str, clang::Expr *expr = nullptr);
    void set_unchecked(std::string str, clang::Expr *expr = nullptr);
    std::string take() {
      auto result = std::move(value);
      value.clear();
      pointee_is_boxed = false;
      return result;
    }
    bool empty() const { return value.empty(); }
    bool is_boxed() const { return pointee_is_boxed; }
    void assert_consumed() const {
      assert(value.empty() && "pending_deref_ not consumed");
    }

  private:
    static bool compute_inner_boxed(clang::Expr *expr);
    std::string value;
    bool pointee_is_boxed = false;
  } pending_deref_;
};
} // namespace cpp2rust
