#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <memory>

#include "converter/converter.h"
#include "converter/models/boxed_vars.h"

namespace cpp2rust {
class ConverterRefCount final : public Converter {
public:
  ConverterRefCount(std::string &rs_code, clang::ASTContext &ctx);
  ConverterRefCount(std::string &rs_code, clang::ASTContext &ctx,
                    std::shared_ptr<const BoxedVars> boxed_vars);

  void EmitFilePreamble() override;

  static void EmitMethodsOnPtr(std::string &out);

  bool VisitRecordType(clang::RecordType *type) override;

  bool VisitConstantArrayType(clang::ConstantArrayType *type) override;

  bool VisitIncompleteArrayType(clang::IncompleteArrayType *type) override;

  bool VisitReferenceType(clang::ReferenceType *type) override;

  bool VisitPointerType(clang::PointerType *type) override;

  std::string
  ConvertFunctionPointerType(const clang::FunctionProtoType *proto) override;

  bool VisitCXXRecordDecl(clang::CXXRecordDecl *decl) override;

  bool VisitOffsetOfExpr(clang::OffsetOfExpr *expr) override;

  void EmitRustUnion(clang::RecordDecl *decl) override;

  bool EmitsReprCForRecords() const override { return false; }

  const char *CharRustType() const override { return "i8"; }

  std::string GetComparisonReferenceArg(const clang::CXXRecordDecl *decl,
                                        std::string_view value) override;

  std::string GetComparisonReceiver(const clang::CXXMethodDecl *method,
                                    const clang::CXXRecordDecl *decl,
                                    std::string_view lhs) override;

  std::string GetShallowCopy(const clang::RecordDecl *decl,
                             std::string_view src);
  void AddCloneTrait(const clang::RecordDecl *decl) override;

  bool RecordImplementsClone(const clang::RecordDecl *decl) override;

  bool RecordDerivesClone(const clang::RecordDecl *decl);

  bool RecordDerivesDeepClone(const clang::RecordDecl *decl);

  void EmitByteSizeAttr(const clang::RecordDecl *decl) override;

  bool
  VisitUnaryExprOrTypeTraitExpr(clang::UnaryExprOrTypeTraitExpr *expr) override;

  void AddDefaultTrait(const clang::RecordDecl *decl) override;

  void AddDefaultTraitForUnion(const clang::RecordDecl *decl) override;

  std::string GetSelfMaybeWithMut(const clang::CXXMethodDecl *decl) override;

  bool ShouldConvertMethod(const clang::CXXMethodDecl *decl) override;

  bool ConvertOutOfLineMethod(clang::CXXMethodDecl *decl) override;

  void ConvertCXXConstructorBody(clang::CXXConstructorDecl *decl) override;

  void ConvertCXXRecordMethods(clang::CXXRecordDecl *decl) override;

  void ConvertLateInstantiatedMethods(clang::CXXRecordDecl *decl) override;

  void ConvertMethodOnPtrTraitDecl(clang::CXXMethodDecl *method);
  void ConvertMethodOnPtr(clang::CXXMethodDecl *method);

  bool VisitCXXThisExpr(clang::CXXThisExpr *expr) override;

  bool ThisIsRustPtr() const override;

  bool VisitCXXConstructorDecl(clang::CXXConstructorDecl *decl) override;

  bool VisitFieldDecl(clang::FieldDecl *decl) override;

  void EmitFunctionPreamble(clang::FunctionDecl *decl) override;

  bool VisitVarDecl(clang::VarDecl *decl) override;
  bool LazyStaticInit() const override { return false; }
  std::string ForceGlobalInit(const clang::VarDecl *decl) override;

  void ConvertGlobalVarDecl(clang::VarDecl *decl) override;

  void ConvertVaListVarDecl(clang::VarDecl *decl) override;

  bool ConvertVarDeclSkipInit(clang::VarDecl *decl) override;

  void EmitHoistedInArmAssignment(clang::VarDecl *decl) override;

  void ConvertDeclRefValue(clang::Expr *expr, clang::ValueDecl *decl) override;

  bool ConvertIncAndDec(clang::UnaryOperator *expr) override;

  bool VisitConditionalOperator(clang::ConditionalOperator *expr) override;

  void ConvertPrintf(clang::CallExpr *expr) override;

  void EmitFnPtrCall(clang::Expr *callee) override;

  bool VisitLambdaExpr(clang::LambdaExpr *expr) override;

  void ConvertCapturelessLambda(const clang::CXXRecordDecl *decl) override;

  const char *LambdaMacro() const override { return "lambda!"; }

  void ConvertLambdaCapture(const clang::FieldDecl *field,
                            clang::Expr *init) override;

  void ConvertLambdaToFunctionPointer(clang::Expr *lambda) override;

  void
  ConvertFunctionToFunctionPointer(const clang::FunctionDecl *fn_decl) override;

  std::string ConvertFnPtrPlaceholder(clang::Expr *arg) override;

  // FnPtr does not implement Copy
  bool FunctionPointerImplementsCopy() const override { return false; }
  bool VisitCallExpr(clang::CallExpr *expr) override;

  bool VisitStringLiteral(clang::StringLiteral *expr) override;

  bool VisitImplicitCastExpr(clang::ImplicitCastExpr *expr) override;

  bool VisitFunctionPointerCast(clang::ExplicitCastExpr *expr);

  bool VisitExplicitCastExpr(clang::ExplicitCastExpr *expr) override;

  void ConvertBinaryOperator(clang::BinaryOperator *expr) override;

  bool VisitStmtExpr(clang::StmtExpr *expr) override;

  bool VisitInitListExpr(clang::InitListExpr *expr) override;

  bool VisitCXXStdInitializerListExpr(
      clang::CXXStdInitializerListExpr *expr) override;

  bool VisitArrayInitLoopExpr(clang::ArrayInitLoopExpr *expr) override;

  bool VisitArraySubscriptExpr(clang::ArraySubscriptExpr *expr) override;

  bool VisitMemberExpr(clang::MemberExpr *expr) override;

  void ConvertUnionMemberAccessor(clang::MemberExpr *expr);

  // Converts an access to a field that is stored inline in its struct.
  void ConvertInlineField(clang::MemberExpr *expr);

  // A pointer to the struct whose field `expr` accesses.
  std::string ConvertRecordPtr(clang::MemberExpr *expr);

  // Copies the value of the field `expr` out of its struct, such that the
  // struct doesn't stay borrowed.
  std::string ReadField(clang::MemberExpr *expr,
                        std::string_view copy = ".clone()");

  // A field that is read through a pointer to its struct is read in a
  // closure, `p.with(|__s| __s.x)`. While converting the struct whose field
  // is read, record_base_, or `*p` for `p->x`, the dereference of a pointer
  // to it is emitted as `__s`, and the pointer is stored in *record_ptr_.
  std::string *record_ptr_ = nullptr;
  const clang::Expr *record_base_ = nullptr;

  // The expression that ConvertFreshRValue copies.
  const clang::Expr *copied_expr_ = nullptr;

  struct PushRecordPtr {
    ConverterRefCount &c;
    std::string *record_ptr;
    const clang::Expr *record_base;

    PushRecordPtr(ConverterRefCount &c, std::string *ptr,
                  const clang::Expr *base)
        : c(c), record_ptr(std::exchange(c.record_ptr_, ptr)),
          record_base(std::exchange(
              c.record_base_, base ? base->IgnoreParenImpCasts() : nullptr)) {}
    ~PushRecordPtr() {
      c.record_ptr_ = record_ptr;
      c.record_base_ = record_base;
    }
  };

  bool VisitCXXNewExpr(clang::CXXNewExpr *expr) override;

  bool VisitCXXDeleteExpr(clang::CXXDeleteExpr *expr) override;

  bool VisitCXXForRangeStmtMap(clang::CXXForRangeStmt *stmt) override;

  bool VisitCXXForRangeStmtVector(clang::CXXForRangeStmt *stmt) override;

  bool VisitCXXForRangeStmtString(clang::CXXForRangeStmt *stmt) override;

  void EmitByValueShadow(const clang::VarDecl *loop_var, std::string box_expr,
                         const std::string &type_override = "");

  std::string ConvertStream(clang::Expr *expr) override;

  bool VisitCXXConstructExpr(clang::CXXConstructExpr *expr) override;

  bool VisitImplicitValueInitExpr(clang::ImplicitValueInitExpr *expr) override;
  bool
  VisitCXXScalarValueInitExpr(clang::CXXScalarValueInitExpr *expr) override;

  bool VisitVAArgExpr(clang::VAArgExpr *expr) override;

  void ConvertVariadicArg(clang::Expr *arg) override;

  void ConvertArrayCXXConstructExpr(clang::CXXConstructExpr *expr) override;

  bool VisitCXXDefaultArgExpr(clang::CXXDefaultArgExpr *expr) override;

  std::string GetDefaultAsString(clang::QualType qual_type) override;

  std::string GetArrayDefaultAsString(clang::QualType qual_type) override;

  void ConvertEqualsNullPtr(clang::Expr *expr) override;

  std::string GetDefaultAsStringFallback(clang::QualType qual_type) override;

  std::string ConvertVarDefaultInit(const clang::VarDecl *decl) override;

  std::vector<const char *>
  GetStructAttributes(const clang::RecordDecl *decl) override;

  bool TypeDerivesDefault(clang::QualType qual_type) override;

  bool Convert(clang::QualType qual_type) override;
  bool
  Convert(clang::Expr *expr,
          std::optional<clang::QualType> implicit_convert_to = {}) override {
    auto *record_ptr = record_ptr_;
    if (!expr || expr->IgnoreParenImpCasts() != record_base_) {
      record_ptr_ = nullptr;
    }
    auto result = Converter::Convert(expr, implicit_convert_to);
    record_ptr_ = record_ptr;
    if (computed_expr_type_ == ComputedExprType::Pending) {
      assert(!pending_deref_.empty() && "pending_deref_ taken without type");
    }
    return result;
  }
  bool Convert(clang::Stmt *stmt) override {
    auto result = Converter::Convert(stmt);
    pending_deref_.assert_consumed();
    return result;
  }

  void ConvertVarInit(clang::QualType qual_type, clang::Expr *expr) override;

  void ConvertFieldInit(const clang::FieldDecl *field,
                        clang::Expr *init) override;

  std::string ConvertVarInitValue(clang::QualType qual_type, clang::Expr *expr);

  void ConvertAssignment(clang::Expr *lhs, clang::Expr *rhs,
                         std::string_view assign_operator) override;

  void ConvertUniquePtrDeref(clang::CXXOperatorCallExpr *expr) override;

  bool ConvertCXXOperatorCallExpr(clang::CXXOperatorCallExpr *expr) override;

  void ConvertFunctionParameters(clang::FunctionDecl *decl) override;

  void ConvertArraySubscript(clang::Expr *base, clang::Expr *idx,
                             clang::QualType type) override;
  // The global array referenced by base when one of its elements is read as
  // an rvalue and can be copied out of the array, or nullptr otherwise.
  clang::DeclRefExpr *GetGlobalArrayRValue(clang::Expr *base);
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
  void SetUFCSReceiver(clang::Expr *base, bool is_arrow,
                       const clang::CXXMethodDecl *method) override;

  std::string GetUFCSName(const clang::CXXMethodDecl *method) const override;

  std::string TraitName(const clang::CXXRecordDecl *decl) const;

  struct MethodsOnPtr {
    DeferredBlock trait;
    DeferredBlock impl;
  };

  // record name -> trait and impl for Ptr<record>, emitted after all
  // translation units.
  static std::map<std::string, MethodsOnPtr> methods_on_ptr_;

  MethodsOnPtr &MethodsOnPtrFor(const clang::CXXRecordDecl *decl);

  std::string DestroyMembers(const clang::CXXRecordDecl *decl) override;

  void EmitScopedDestructor(const clang::VarDecl *decl) override;

  std::pair<std::string, std::string>
  MaterializeTemp(const std::string &binding_name, clang::QualType param_type,
                  clang::Expr *expr) override;

  void ConvertConstructedValue(clang::QualType type,
                               clang::CXXConstructExpr *ctor) override;

  const char *GetPointerDerefSuffix(clang::QualType pointee_type);
  const char *GetPointerDerefPrefix(clang::QualType pointee_type) override;

  void EmitSetOrAssign(clang::Expr *lhs, std::string_view rhs);

  // Whether an assigned value can be wrapped in braces to release its borrows
  // before lhs is borrowed, as Rust 2024 drops the temporaries of a block's
  // tail expression at the end of the block. This requires the value to be
  // evaluated before lhs (or the order to be unobservable); otherwise it must
  // be bound to a variable first.
  bool CanBraceAssignedValue(clang::Expr *lhs, clang::Expr *rhs,
                             std::string_view assign_operator);

  // Emits `lhs = value`, where value is the result of the compound assignment
  // `lhs op= rhs` and thus reads lhs.
  void EmitCompoundSetOrAssign(clang::Expr *lhs, clang::Expr *rhs,
                               std::string_view value);

  // If lhs is a direct reference to a global/static value (not a reference
  // type), emits `var.with(|rc| *rc.borrow_mut() <op> <rhs>)` and returns
  // true. This avoids cloning the Rc just to assign through it. Returns
  // false (emitting nothing) if lhs doesn't match this shape.
  bool EmitGlobalValueAssign(clang::Expr *lhs, std::string_view assign_operator,
                             std::string_view rhs);

  // Wraps a pointer expression with deref prefix/suffix: e.g.
  // "(*ptr.upgrade().deref())" or "(ptr.read())"
  std::string DerefPtrExpr(std::string_view ptr_expr,
                           clang::QualType pointee_type);

  std::string GetInnerType(clang::QualType type);

  std::string ConvertFreshLValue(clang::Expr *expr);
  // What an Object-kind conversion of a boxed container/array should yield:
  // a pointer to the whole object (Ptr<Vec<T>>), or one to its first
  // element (Ptr<T>).
  enum class ObjectShape { Whole, Element };
  std::string ConvertObject(clang::Expr *expr,
                            ObjectShape shape = ObjectShape::Whole);
  std::string
  ConvertFreshObject(clang::Expr *expr,
                     std::string_view target_ptr_type = {}) override;
  bool WantsElementPtr() const { return object_shape_ == ObjectShape::Element; }
  std::string
  ConvertFresh(clang::Expr *expr,
               std::optional<clang::QualType> implicit_convert_to = {});
  std::string ConvertFreshRValue(
      clang::Expr *expr,
      std::optional<clang::QualType> implicit_convert_to = {}) override;
  std::string ConvertFreshPointer(
      clang::Expr *expr,
      std::optional<clang::QualType> implicit_convert_to = {}) override;

  std::string ConvertPtrType(clang::QualType type);
  std::string ConvertPointeeType(clang::QualType ptr_type) override;

  void ConvertParamTyConstCast(clang::QualType, clang::Expr *) override {}

  std::string ConvertPointeeCast(std::string str, const clang::Expr *from,
                                 clang::QualType to) override;

  std::string ConvertSubscriptIndex(clang::Expr *idx);

  // Element `idx` of pointer `base`, accessed without creating a pointer to it.
  void ConvertPointerElem(clang::Expr *base, clang::Expr *idx);

  std::string GetSafeTypeAsString(clang::QualType qual_type) const;

  bool NeedsMut(const clang::VarDecl *decl, clang::QualType type,
                llvm::StringRef /*name*/) const override;

  // Whether decl is a local variable or a parameter that is stored directly
  // instead of in a Value, as no pointer to it is ever made.
  bool IsUnboxedVar(const clang::ValueDecl *decl) const;
  std::shared_ptr<const BoxedVars> boxed_vars_;

  /// The kind of conversion that should be performed.
  enum class ConversionKind : uint8_t {
    Unboxed,
    Pointee,
    Ptr,
    FullRefCount,
  };

  static const char *ConversionKindToString(ConversionKind k) {
    switch (k) {
    case ConversionKind::Unboxed:
      return "Unboxed";
    case ConversionKind::Pointee:
      return "Pointee";
    case ConversionKind::Ptr:
      return "Ptr";
    case ConversionKind::FullRefCount:
      return "FullRefCount";
    }
    std::unreachable();
  }

  ConversionKind getConversionKind() const { return conversion_kind_.back(); }

  // How a variable is stored.
  ConversionKind VarConversionKind(const clang::VarDecl *decl) const;

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

  std::string BoxType(std::string &&str) const;
  std::string BoxValue(std::string &&str) const;

  std::vector<ConversionKind> conversion_kind_;
  ObjectShape object_shape_ = ObjectShape::Whole;

  // Set by pointer-related visit methods (ConvertDeref,
  // ConvertPointerSubscript, etc.) when converting an LValue that goes through
  // a Ptr. Contains the ptr expression string. Consumed by EmitSetOrAssign to
  // emit ptr.write(rhs), or by ConvertMappedMethodCall to emit
  // ptr.with_mut(...).
  struct PendingDeref {
    PendingDeref(ComputedExprType &type, clang::ASTContext &ctx)
        : type(type), ctx(ctx) {}
    void set(std::string str, bool fresh, clang::Expr *expr = nullptr);
    void set_unchecked(std::string str, bool fresh,
                       clang::Expr *expr = nullptr);
    std::string take() {
      auto result = std::move(value);
      value.clear();
      pointee_is_boxed = false;
      ptr_is_fresh = false;
      return result;
    }
    bool empty() const { return value.empty(); }
    bool is_boxed() const { return pointee_is_boxed; }
    bool is_fresh() const { return ptr_is_fresh; }
    void assert_consumed() const {
      assert(value.empty() && "pending_deref_ not consumed");
    }

  private:
    bool compute_inner_boxed(clang::Expr *expr) const;
    ComputedExprType &type;
    clang::ASTContext &ctx;
    std::string value;
    bool pointee_is_boxed = false;
    bool ptr_is_fresh = false;
  } pending_deref_{computed_expr_type_, ctx_};
};
} // namespace cpp2rust
