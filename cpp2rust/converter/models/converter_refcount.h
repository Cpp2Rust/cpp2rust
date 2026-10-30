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

  std::string GetUnboxedTypeAsString(clang::QualType qual_type) override;

  std::string GetUnboxedDefaultAsString(clang::QualType qual_type) override;

  static void EmitMethodsOnPtr(std::string &out);

  bool VisitRecordType(clang::RecordType *type) override;

  bool VisitConstantArrayType(clang::ConstantArrayType *type) override;

  bool VisitIncompleteArrayType(clang::IncompleteArrayType *type) override;

  bool VisitReferenceType(clang::ReferenceType *type) override;

  bool VisitPointerType(clang::PointerType *type) override;

  std::string
  ConvertFunctionPointerType(const clang::FunctionProtoType *proto) override;

  bool VisitOffsetOfExpr(clang::OffsetOfExpr *expr) override;

  void EmitRustUnion(clang::RecordDecl *decl) override;

  bool EmitsReprCForRecords() const override { return false; }

  const char *CharRustType() const override { return "i8"; }

  std::string GetComparisonArg(const clang::ParmVarDecl *param,
                               const clang::CXXRecordDecl *decl,
                               std::string_view value) override;

  std::string GetComparisonResultReceiver(const clang::CXXMethodDecl *method,
                                          bool returns_ref,
                                          std::string_view result) override;

  std::string GetComparisonReceiver(const clang::CXXMethodDecl *method,
                                    std::string_view lhs) override;

  void ConvertOrdAndPartialOrdTraits(const clang::CXXRecordDecl *decl,
                                     const clang::FunctionDecl *eq,
                                     const clang::FunctionDecl *lt,
                                     const clang::FunctionDecl *cmp) override;

  std::string GetShallowCopy(const clang::RecordDecl *decl,
                             std::string_view src);
  void AddCloneTrait(const clang::RecordDecl *decl) override;

  bool RecordImplementsClone(const clang::RecordDecl *decl) override;

  bool RecordDerivesClone(const clang::RecordDecl *decl);

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

  bool ConvertVarDeclSkipInit(clang::VarDecl *decl) override;

  void EmitHoistedInArmAssignment(clang::VarDecl *decl) override;

  void ConvertDeclRefValue(clang::Expr *expr, clang::ValueDecl *decl) override;

  bool ConvertIncAndDec(clang::UnaryOperator *expr) override;

  void ConvertConditionalBranch(clang::Expr *expr,
                                clang::QualType type) override;

  void ConvertPrintf(clang::CallExpr *expr);

  void ConvertVAArgCall(clang::CallExpr *expr) override;

  std::optional<TempMaterializationCtx>
  ConvertCallExpr(clang::CallExpr *expr) override;

  void EmitCallResult(clang::CallExpr *expr, std::string str,
                      const TempMaterializationCtx *temps) override;

  void EmitFnPtrCall(clang::CallExpr *expr) override;

  bool VisitLambdaExpr(clang::LambdaExpr *expr) override;

  void ConvertCapturelessLambda(const clang::CXXRecordDecl *decl) override;

  const char *LambdaMacro() const override { return "lambda!"; }

  void ConvertLambdaCapture(const clang::FieldDecl *field,
                            clang::Expr *init) override;

  void ConvertLambdaToFunctionPointer(
      clang::Expr *lambda, const clang::CXXConversionDecl *conversion) override;

  void
  ConvertFunctionToFunctionPointer(const clang::FunctionDecl *fn_decl) override;

  std::string ConvertFnPtrPlaceholder(clang::Expr *arg) override;

  // FnPtr does not implement Copy
  bool FunctionPointerImplementsCopy() const override { return false; }

  bool VisitStringLiteral(clang::StringLiteral *expr) override;

  std::string GetSourceFileAsString(clang::QualType type) override;

  bool VisitImplicitCastExpr(clang::ImplicitCastExpr *expr) override;

  // Converts a cast from or to a function pointer, if expr is one.
  bool ConvertFunctionPointerCast(clang::ExplicitCastExpr *expr);

  bool VisitExplicitCastExpr(clang::ExplicitCastExpr *expr) override;

  void ConvertBinaryOperator(clang::BinaryOperator *expr) override;

  bool VisitStmtExpr(clang::StmtExpr *expr) override;
  bool VisitReturnStmt(clang::ReturnStmt *stmt) override;

  void ConvertScalarInitList(clang::InitListExpr *expr) override;
  void ConvertRecordInitList(clang::InitListExpr *expr, bool is_empty) override;
  void ConvertUnionInitList(clang::InitListExpr *expr,
                            const clang::FieldDecl *field) override;
  void ConvertArrayInitList(clang::InitListExpr *expr) override;

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

  // The reads of unboxed variables in the returned expression being
  // converted that are moved instead of copied, as the variables die on
  // return.
  std::unordered_set<const clang::DeclRefExpr *> moved_reads_;

  struct PushRecordPtr {
    ScopedValue<std::string *> record_ptr;
    ScopedValue<const clang::Expr *> record_base;

    PushRecordPtr(ConverterRefCount &c, std::string *ptr,
                  const clang::Expr *base)
        : record_ptr(c.record_ptr_, ptr),
          record_base(c.record_base_,
                      base ? base->IgnoreParenImpCasts() : nullptr) {}
  };

  bool VisitCXXNewExpr(clang::CXXNewExpr *expr) override;

  bool VisitCXXDeleteExpr(clang::CXXDeleteExpr *expr) override;

  std::string ConvertStream(clang::Expr *expr) override;

  bool VisitCXXConstructExpr(clang::CXXConstructExpr *expr) override;

  bool VisitImplicitValueInitExpr(clang::ImplicitValueInitExpr *expr) override;
  bool
  VisitCXXScalarValueInitExpr(clang::CXXScalarValueInitExpr *expr) override;

  bool VisitVAArgExpr(clang::VAArgExpr *expr) override;

  void ConvertVariadicArg(clang::Expr *arg) override;

  void ConvertArrayCXXConstructExpr(clang::CXXConstructExpr *expr) override;

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

  bool ConvertCXXOperatorCallExpr(clang::CXXOperatorCallExpr *expr) override;

  void ConvertFunctionParameters(clang::FunctionDecl *decl) override;

  void ConvertArraySubscript(clang::Expr *base, clang::Expr *idx,
                             clang::QualType type) override;
  // The global array referenced by base when one of its elements is read as
  // an rvalue and can be copied out of the array, or nullptr otherwise.
  clang::DeclRefExpr *GetGlobalArrayRValue(clang::Expr *base);
  void ConvertPointerSubscript(clang::ArraySubscriptExpr *expr) override;

  bool IsPointerSubscript(const clang::ArraySubscriptExpr *expr) const;

  void ConvertFunctionMain(const clang::FunctionDecl *decl,
                           const std::string_view main_function_name) override;

  void ConvertAddrOf(clang::Expr *expr, clang::QualType pointer_type) override;

  void ConvertDeref(clang::Expr *expr) override;

  void ConvertArrow(clang::Expr *expr) override;

  std::string AccessLValueObject(clang::MemberExpr *member) override;

  void ConvertGenericBinaryOperator(clang::BinaryOperator *expr) override;

  bool IsReferenceType(const clang::Expr *expr) const override;

  std::string ConvertTake(clang::Expr *arg) override;

  std::string ConvertBorrowMut(clang::Expr *arg) override;

  // Whether the receiver of a method call translated by a rule is being
  // converted.
  bool in_receiver_ = false;

  std::string ConvertMappedMethodCall(
      clang::Expr *expr, const TranslationRule::MethodCallFragment &mc,
      llvm::ArrayRef<clang::TemplateArgument> bindings, clang::Expr **args,
      unsigned num_args, TempMaterializationCtx *ctx) override;

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

  // Whether a pointer to pointee_type is dereferenced with .read(), which
  // returns a copy of the pointee, rather than a borrow of it.
  bool DerefReadsValue(clang::QualType pointee_type);
  const char *GetPointerDerefSuffix(clang::QualType pointee_type);
  // Sets the freshness of the dereference of a pointer to pointee_type: a
  // copy read with .read() is fresh.
  void SetDerefFreshness(clang::QualType pointee_type);
  const char *GetPointerDerefPrefix(clang::QualType pointee_type) override;

  // Dereferences, if enabled, the pointer to pointee_type emitted in scope.
  class PushDeref {
    ConverterRefCount &c;
    clang::QualType pointee_type;
    bool enabled;

  public:
    PushDeref(ConverterRefCount &c, clang::QualType pointee_type,
              bool enabled = true)
        : c(c), pointee_type(pointee_type), enabled(enabled) {
      if (enabled) {
        c.StrCat(token::kOpenParen, c.GetPointerDerefPrefix(pointee_type));
      }
    }
    ~PushDeref() {
      if (enabled) {
        c.StrCat(c.GetPointerDerefSuffix(pointee_type), token::kCloseParen);
        c.SetDerefFreshness(pointee_type);
      }
    }
    PushDeref(const PushDeref &) = delete;
    PushDeref &operator=(const PushDeref &) = delete;
  };

  // Emits the object that ptr points to: a pending dereference, which is
  // written through ptr, if an lvalue is converted, or a read otherwise.
  void EmitPointee(std::string ptr, clang::QualType pointee_type, bool fresh);

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
                           clang::QualType pointee_type) override;

  const char *GetConversionTraitName() const override;

  std::string GetInnerType(clang::QualType type);

  // What an Object-kind conversion of a boxed container/array should yield:
  // a pointer to the whole object (Ptr<Vec<T>>), or one to its first
  // element (Ptr<T>).
  enum class ObjectShape { Whole, Element };
  std::string ConvertObject(clang::Expr *expr,
                            ObjectShape shape = ObjectShape::Whole);
  std::string ConvertFreshObject(clang::Expr *expr) override;

  std::string ConvertPointerReceiver(clang::Expr *arg,
                                     std::string_view param_type) override;

  bool ParamIsElementPtr(clang::Expr *expr, clang::Expr *arg,
                         unsigned index) override;
  std::string ConvertElementPtr(clang::Expr *arg) override;
  // The method that makes a pointer to an object of the given type, which is
  // one to its elements if they are what ConvertObject wants.
  std::string AsPointerMethod(clang::QualType type);
  void CheckRuleReturnType(clang::Expr *expr) override;

  std::string ConvertTemporaryPointer(
      clang::Expr *expr,
      std::optional<clang::QualType> implicit_convert_to) override;

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

  // A pointer to the first element of the string literal expr.
  std::string ConvertStringLiteralPtr(clang::Expr *expr);

  // Element `idx` of pointer `base`, accessed without creating a pointer to it.
  void ConvertPointerElem(clang::Expr *base, clang::Expr *idx);

  // The place of element `idx` of the container `container`, e.g., a
  // std::vector.
  std::string ConvertElemPlace(clang::Expr *container, clang::Expr *idx);

  // Reads the element at place, a record, for a read of one of its fields in
  // a closure that borrows it, or as a whole.
  void EmitRecordElemRead(std::string_view place, clang::QualType type);

  // Captures what convert emits as the pending dereference.
  void CapturePendingDeref(llvm::function_ref<void()> convert);

  // The place of `expr`, which is reached through a pointer.
  std::string ConvertPlace(clang::Expr *expr);

  std::string GetSafeTypeAsString(clang::QualType qual_type) const;

  bool NeedsMut(const clang::VarDecl *decl, clang::QualType type,
                llvm::StringRef /*name*/) const override;

  // Whether decl is a local variable or a parameter that is stored directly
  // instead of in a Value, as no pointer to it is ever made.
  bool IsUnboxedVar(const clang::ValueDecl *decl) const;
  // Whether expr is an unboxed variable or a field of one, which is accessed
  // in place.
  bool IsUnboxedPlace(const clang::Expr *expr) const;
  // Whether expr is a local variable or parameter stored in a Value.
  bool IsValueLocal(const clang::Expr *expr) const;

  bool IsFieldOfVectorElem(const clang::MemberExpr *expr) const;

  bool IsElemOfBorrowedValue(const clang::CXXOperatorCallExpr *expr) const;
  // Whether expr is a field of an unboxed variable, or of a field of one,
  // which is stored in place, even if it is a reference.
  bool IsFieldOfUnboxedPlace(const clang::MemberExpr *expr) const;
  std::shared_ptr<const BoxedVars> boxed_vars_;

  // Set when a constructor's translation refers to `this`, which then needs
  // the object being built to be in a Value.
  bool ctor_uses_this_ = false;

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

  std::string BoxType(std::string &&str) const;
  std::string BoxValue(std::string &&str) const;

  std::vector<ConversionKind> conversion_kind_;
  // The shape that the innermost ConvertObject wants, until a visitor makes a
  // pointer of that shape.
  ObjectShape object_shape_ = ObjectShape::Whole;

  // Set by pointer-related visit methods (ConvertDeref,
  // ConvertPointerSubscript, etc.) when converting an LValue that goes through
  // a Ptr. Contains the ptr expression string. Consumed by EmitSetOrAssign to
  // emit ptr.write(rhs), or by ConvertMappedMethodCall to emit
  // ptr.with_mut(...).
  struct PendingDeref {
    PendingDeref(ComputedExprType &type) : type(type) {}
    void set(std::string str, bool fresh);
    void set_unchecked(std::string str, bool fresh);
    std::string take() {
      auto result = std::move(value);
      value.clear();
      ptr_is_fresh = false;
      return result;
    }
    bool empty() const { return value.empty(); }
    bool is_fresh() const { return ptr_is_fresh; }
    void assert_consumed() const {
      assert(value.empty() && "pending_deref_ not consumed");
    }

  private:
    ComputedExprType &type;
    std::string value;
    bool ptr_is_fresh = false;
  } pending_deref_{computed_expr_type_};
};
} // namespace cpp2rust
