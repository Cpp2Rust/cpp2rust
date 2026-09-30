#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/converter.h"

namespace cpp2rust {
class ConverterRefCount final : public Converter {
public:
  ConverterRefCount(std::string &rs_code, clang::ASTContext &ctx);

  void EmitFilePreamble() override;

  static void EmitMethodsOnPtr(std::string &out);

  bool VisitRecordType(clang::RecordType *type) override;

  bool VisitConstantArrayType(clang::ConstantArrayType *type) override;

  bool VisitIncompleteArrayType(clang::IncompleteArrayType *type) override;

  bool VisitReferenceType(clang::ReferenceType *type) override;

  bool VisitPointerType(clang::PointerType *type) override;

  std::string
  ConvertFunctionPointerType(const clang::FunctionProtoType *proto,
                             FnProtoType kind = FnProtoType::FnPtr) override;

  bool VisitCXXRecordDecl(clang::CXXRecordDecl *decl) override;

  bool VisitOffsetOfExpr(clang::OffsetOfExpr *expr) override;

  void EmitRustUnion(clang::RecordDecl *decl) override;

  bool EmitsReprCForRecords() const override { return false; }

  const char *CharRustType() const override { return "u8"; }

  std::string GetComparisonReferenceArg(const clang::CXXRecordDecl *decl,
                                        std::string_view value) override;

  std::string GetComparisonReceiver(const clang::CXXMethodDecl *method,
                                    const clang::CXXRecordDecl *decl,
                                    std::string_view lhs) override;

  std::string GetShallowCopy(const clang::RecordDecl *decl,
                             std::string_view src);
  void AddCloneTrait(const clang::RecordDecl *decl) override;

  bool RecordImplementsClone(const clang::RecordDecl *decl) override;

  std::string GetDeepCopy(clang::QualType type, std::string_view src);

  // Whether the Clone of a struct copies each field with its own clone(), and
  // hence can be derived.
  bool RecordDerivesClone(const clang::RecordDecl *decl);

  void AddByteReprTrait(const clang::RecordDecl *decl) override;

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

  bool ConvertLambdaVarDecl(clang::VarDecl *decl) override;

  void ConvertDeclRefValue(clang::Expr *expr, clang::ValueDecl *decl) override;

  bool ConvertIncAndDec(clang::UnaryOperator *expr) override;

  bool VisitConditionalOperator(clang::ConditionalOperator *expr) override;

  void ConvertPrintf(clang::CallExpr *expr) override;

  void EmitFnPtrCall(clang::Expr *callee) override;

  void
  ConvertFunctionToFunctionPointer(const clang::FunctionDecl *fn_decl) override;

  std::string ConvertFnPtrPlaceholder(clang::Expr *arg) override;

  // FnPtr does not implement Copy
  bool FunctionPointerImplementsCopy() const override { return false; }
  bool FunctionPointerCastNeedsTransmute() const override { return false; }

  bool VisitCallExpr(clang::CallExpr *expr) override;

  bool VisitStringLiteral(clang::StringLiteral *expr) override;

  bool VisitImplicitCastExpr(clang::ImplicitCastExpr *expr) override;

  bool VisitFunctionPointerCast(clang::ExplicitCastExpr *expr);

  bool VisitExplicitCastExpr(clang::ExplicitCastExpr *expr) override;

  void ConvertBinaryOperator(clang::BinaryOperator *expr) override;

  bool VisitStmtExpr(clang::StmtExpr *expr) override;

  void EmitStmtExprTail(clang::Expr *tail) override;

  bool VisitInitListExpr(clang::InitListExpr *expr) override;

  bool VisitCXXStdInitializerListExpr(
      clang::CXXStdInitializerListExpr *expr) override;

  bool VisitArrayInitLoopExpr(clang::ArrayInitLoopExpr *expr) override;

  bool VisitArraySubscriptExpr(clang::ArraySubscriptExpr *expr) override;

  // Converts the subscript expr with convert, having evaluated its index
  // first, if the index has side effects that may conflict with the borrow of
  // the struct that holds base. Returns false if it doesn't do so.
  bool ConvertWithHoistedIndex(clang::Expr *expr, clang::Expr *base,
                               clang::Expr *idx,
                               llvm::function_ref<void()> convert);

  bool VisitMemberExpr(clang::MemberExpr *expr) override;

  void ConvertUnionMemberAccessor(clang::MemberExpr *expr);

  // Converts an access to a field of a struct translated by us. Fields are
  // stored inline in the struct.
  void ConvertFieldAccess(clang::MemberExpr *expr);

  // The struct whose field `expr` accesses, as a place expression. The struct
  // is borrowed mutably if `mut` is set.
  std::string ConvertRecordPlace(clang::MemberExpr *expr, bool mut);

  // Wraps ptr, the result of converting expr, such that it doesn't keep any
  // struct borrowed if it is written through.
  std::string DetachPtr(std::string ptr, bool fresh, const clang::Expr *expr);
  std::string TakePendingDerefToWrite(const clang::Expr *lvalue);

  // A Ptr to the field accessed by `expr`.
  std::string ConvertFieldPointer(clang::MemberExpr *expr);

  // The Value held by a Value field (see IsValueField).
  std::string ConvertValueField(clang::MemberExpr *expr);

  // Converts the field `expr` given the place where it is stored.
  void ConvertFieldPlace(clang::MemberExpr *expr, std::string place);

  // A field of a struct reached through the pointer `ptr`, which a place
  // expression accesses (e.g., p->x, (*p).x, or r.a[i].x for a reference r).
  // `nested` if the place is converted inside ConvertInStructPtr already.
  struct StructPtr {
    clang::MemberExpr *member;
    clang::Expr *ptr = nullptr;
    bool via_reference = false;
    bool nested = false;
    // Whether the place is in a Value field, which is borrowed separately
    // from the struct.
    bool in_value_field = false;
  };
  // If `mut`, the place is written, and must not borrow the struct.
  std::optional<StructPtr> FindStructPtr(clang::Expr *place, bool mut = false);

  // If `place` is a field of a struct reached through a pointer, converts
  // the expression given by `convert`, which accesses `place`, in a closure
  // that borrows the struct through the pointer, `p.with(|__s: &S| ...)`.
  std::optional<std::string>
  ConvertInStructPtr(clang::Expr *place, bool mut,
                     llvm::function_ref<std::string()> convert);

  std::string ConvertTake(clang::Expr *arg) override {
    auto take = [&] { return Converter::ConvertTake(arg); };
    auto str = ConvertInStructPtr(arg, true, take);
    return str ? *str : take();
  }

  // The fields converted by ConvertInStructPtr, and their places in the
  // closure.
  std::unordered_map<const clang::Expr *, std::string> field_places_;

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

  std::string ConvertVarDefaultInit(clang::QualType qual_type) override;

  std::vector<const char *>
  GetStructAttributes(const clang::RecordDecl *decl) override;

  bool Convert(clang::QualType qual_type) override;
  bool
  Convert(clang::Expr *expr,
          std::optional<clang::QualType> implicit_convert_to = {}) override {
    if (auto it = hoisted_exprs_.find(expr); it != hoisted_exprs_.end()) {
      StrCat(it->second);
      computed_expr_type_ = ComputedExprType::FreshValue;
      return false;
    }
    if (auto it = field_places_.find(expr); it != field_places_.end()) {
      ConvertFieldPlace(clang::cast<clang::MemberExpr>(expr), it->second);
      return false;
    }
    // Values read from fields of structs reached through pointers.
    if (isRValue() && TypeIsCopyable(expr->getType())) {
      if (auto str = ConvertInStructPtr(expr, false, [&] {
            return ToString(expr, implicit_convert_to);
          })) {
        StrCat(*str);
        SetFreshType(expr->getType());
        return false;
      }
    }
    auto result = Converter::Convert(expr, implicit_convert_to);
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

  // Converts a rule that calls a method on `receiver_ph`, by `convert`, in
  // a closure if the receiver is a field of a struct reached through a
  // pointer, or after evaluating the arguments that read fields if it is a
  // field.
  std::optional<std::string> ConvertOnFieldReceiver(
      const std::vector<TranslationRule::BodyFragment> &fragments,
      const TranslationRule::PlaceholderFragment &receiver_ph,
      clang::Expr *expr, clang::Expr **args, unsigned num_args,
      llvm::function_ref<std::string()> convert);

  std::string
  ConvertIRFragment(const std::vector<TranslationRule::BodyFragment> &fragments,
                    clang::Expr *expr, clang::Expr **args, unsigned num_args,
                    TempMaterializationCtx *ctx) override;

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

  // Converts `expr` for use where a `qual_type` function pointer is
  // expected, inserting a `.cast()` if `expr`'s own fn pointer type differs
  // from `qual_type` -- e.g. because the two describe the same C function
  // pointer type through different typedefs that the translation maps to
  // distinct Rust types (`size_t` vs `unsigned long`).
  std::string ConvertFnPtrValue(clang::QualType qual_type, clang::Expr *expr);

  void EmitSetOrAssign(clang::Expr *lhs, std::string_view rhs);

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
  std::string ConvertFreshPointer(clang::Expr *expr) override;

  std::string ConvertPtrType(clang::QualType type);
  std::string ConvertPointeeType(clang::QualType ptr_type) override;

  void ConvertParamTyPointerCastIfNeeded(clang::QualType param_type,
                                         clang::Expr *expr) override;

  std::string ConvertSubscriptIndex(clang::Expr *idx);

  std::string GetSafeTypeAsString(clang::QualType qual_type) const;

  bool NeedsMut(const clang::VarDecl *decl, clang::QualType type,
                llvm::StringRef /*name*/) const override;

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
