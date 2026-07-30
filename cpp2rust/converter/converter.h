#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/Sema/Sema.h>

#include <functional>
#include <optional>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

#include "converter/converter_lib.h"
#include "converter/lex.h"
#include "converter/rs_expr.h"
#include "converter/translation_rule.h"
#include "logging.h"

namespace cpp2rust {
class Converter {

public:
  explicit Converter(std::string &rs_code, clang::ASTContext &ctx,
                     const char *keyword_unsafe = "unsafe",
                     const char *keyword_mut = "mut",
                     const char *keyword_const_fn = keyword::kConst)
      : rs_code_(&rs_code), ctx_(ctx), keyword_unsafe_(keyword_unsafe),
        keyword_mut_(keyword_mut), keyword_const_fn_(keyword_const_fn) {}

  virtual ~Converter() = default;

  Converter(const Converter &) = delete;
  Converter &operator=(const Converter &) = delete;
  Converter(Converter &&) = delete;
  Converter &operator=(Converter &&) = delete;

  void SetSema(clang::Sema &sema) { sema_ = &sema; }

  auto &GetSema() {
    assert(sema_ && "sema_ should already be set");
    return *sema_;
  }

  virtual bool Convert(clang::Decl *decl);

  RsExpr *VisitRecoveryExpr(clang::RecoveryExpr *expr);

  virtual std::string EmitFilePreamble();

  static std::string EmitOpaqueRecords();

  virtual RsExpr *Convert(clang::QualType qual_type);

  virtual RsExpr *VisitBuiltinType(const clang::BuiltinType *type);

  virtual RsExpr *VisitRecordType(const clang::RecordType *type);

  virtual RsExpr *VisitConstantArrayType(const clang::ConstantArrayType *type);

  virtual RsExpr *
  VisitIncompleteArrayType(const clang::IncompleteArrayType *type);

  virtual RsExpr *
  VisitLValueReferenceType(const clang::LValueReferenceType *type);

  virtual RsExpr *VisitDecayedType(const clang::DecayedType *type);

  virtual RsExpr *VisitPointerType(const clang::PointerType *type);

  virtual RsExpr *VisitTypedefType(const clang::TypedefType *type);

  virtual RsExpr *VisitUsingType(const clang::UsingType *type);

  enum class FnProtoType { LambdaCallOperator, FnPtr };

  virtual RsExpr *
  ConvertFunctionPointerType(const clang::FunctionProtoType *proto,
                             FnProtoType kind = FnProtoType::FnPtr);

  virtual RsExpr *VisitTranslationUnitDecl(clang::TranslationUnitDecl *decl);

  virtual RsExpr *VisitFunctionDecl(clang::FunctionDecl *decl);

  virtual RsExpr *EmitFunctionPreamble(clang::FunctionDecl *decl);

  virtual RsExpr *ConvertFunctionBody(clang::FunctionDecl *decl);

  RsExpr *ConvertGotoBlock(clang::CompoundStmt *body);

  RsExpr *EmitHoistedDecls(clang::CompoundStmt *body);

  virtual RsExpr *VisitFunctionTemplateDecl(clang::FunctionTemplateDecl *decl);

  virtual RsExpr *VisitVarDecl(clang::VarDecl *decl);

  RsExpr *ConvertVarDecl(clang::VarDecl *decl);

  virtual RsExpr *EmitHoistedInArmAssignment(clang::VarDecl *decl);

  RsExpr *ConvertVarDeclInitializer(clang::VarDecl *decl);

  virtual RsExpr *ConvertGlobalVarDecl(clang::VarDecl *decl);

  virtual RsExpr *ConvertVaListVarDecl(clang::VarDecl *decl);

  virtual std::pair<RsExpr *, bool>
  ConvertVarDeclSkipInit(clang::VarDecl *decl);

  virtual bool ConvertLambdaVarDecl(clang::VarDecl *decl);

  RsExpr *VisitRecordDecl(clang::RecordDecl *decl);

  virtual RsExpr *VisitCXXRecordDecl(clang::CXXRecordDecl *decl);

  virtual RsExpr *EmitRustStructOrUnion(clang::RecordDecl *decl);

  virtual RsExpr *EmitRustUnion(clang::RecordDecl *decl);

  virtual bool EmitsReprCForRecords() const { return true; }

  virtual const char *CharRustType() const { return "libc::c_char"; }

  virtual RsExpr *VisitCXXMethodDecl(clang::CXXMethodDecl *decl);
  virtual std::string GetSelfMaybeWithMut(const clang::CXXMethodDecl *decl);

  RsExpr *ConvertCXXConstructorBody(clang::CXXConstructorDecl *decl);

  virtual RsExpr *VisitCXXConstructorDecl(clang::CXXConstructorDecl *decl);

  virtual RsExpr *VisitFieldDecl(clang::FieldDecl *decl);

  virtual RsExpr *VisitNamespaceDecl(clang::NamespaceDecl *decl);

  virtual RsExpr *VisitTypedefDecl(clang::TypedefDecl *decl);

  virtual RsExpr *VisitCompoundStmt(clang::CompoundStmt *stmt);

  virtual RsExpr *VisitDeclStmt(clang::DeclStmt *stmt);

  virtual RsExpr *VisitReturnStmt(clang::ReturnStmt *stmt);

  virtual RsExpr *VisitGotoStmt(clang::GotoStmt *stmt);

  RsExpr *ConvertCondition(clang::Expr *cond);

  virtual RsExpr *VisitIfStmt(clang::IfStmt *stmt);

  virtual RsExpr *VisitWhileStmt(clang::WhileStmt *stmt);

  virtual RsExpr *VisitDoStmt(clang::DoStmt *stmt);

  virtual RsExpr *VisitForStmt(clang::ForStmt *stmt);

  virtual RsExpr *VisitCXXForRangeStmt(clang::CXXForRangeStmt *stmt);

  virtual RsExpr *VisitCXXForRangeStmtMap(clang::CXXForRangeStmt *stmt);

  virtual RsExpr *VisitCXXForRangeStmtVector(clang::CXXForRangeStmt *stmt);

  virtual RsExpr *VisitCXXForRangeStmtString(clang::CXXForRangeStmt *stmt);

  RsExpr *VisitCXXForRangeStmtIndexBased(clang::CXXForRangeStmt *stmt,
                                         const char *len_suffix);

  RsExpr *ConvertForRangeBody(clang::CXXForRangeStmt *stmt,
                              const clang::VarDecl *map_iter_decl = nullptr);

  virtual RsExpr *VisitBreakStmt(clang::BreakStmt *stmt);

  virtual RsExpr *VisitContinueStmt(clang::ContinueStmt *stmt);

  bool GetFmtArg(clang::Expr *arg, std::string &fmt,
                 std::vector<RsExpr *> &fmt_args, const char *&fmt_trait,
                 std::string &fmt_width);

  bool GetRawArg(clang::Expr *arg, std::vector<RsExpr *> &raw_args);

  RsExpr *ConvertCallToOstream(clang::CallExpr *expr);
  virtual RsExpr *ConvertStream(clang::Expr *expr);

  struct TempMaterializationCtx {
    std::vector<std::optional<clang::QualType>> materialized_args;
    std::vector<RsExpr *> temporary_bindings;

    TempMaterializationCtx(size_t num_args)
        : materialized_args(num_args), materialized_refs_(num_args) {}

    RsExpr *GetOrMaterialize(unsigned argument_num,
                             std::function<std::pair<RsExpr *, RsExpr *>(
                                 const std::string &, clang::QualType)>
                                 materialize_fn);

  private:
    std::vector<RsExpr *> materialized_refs_;
  };

  struct PlaceholderCtx {
    std::string param_type;
    std::optional<clang::QualType> implicit_convert_to;
    TempMaterializationCtx *materialize_ctx;
    int materialize_idx; // <0 = no idx, >=0 idx valid
    TranslationRule::Access access;
    bool is_receiver;
    bool is_cpp_ptr;
    bool maps_to_rust_ptr;
    bool declared_in_rule_as_rust_ptr;
    bool is_index_base;

    bool needs_materialization() const {
      return materialize_ctx && materialize_idx >= 0 &&
             declared_in_rule_as_rust_ptr && !is_cpp_ptr && !maps_to_rust_ptr;
    }

    bool needs_pointer_receiver() const {
      return is_receiver && !maps_to_rust_ptr && declared_in_rule_as_rust_ptr;
    }

    bool needs_object_receiver() const {
      return is_receiver && is_cpp_ptr && !declared_in_rule_as_rust_ptr;
    }

    bool needs_ptr_wrap() const {
      return !is_receiver && !is_cpp_ptr && !maps_to_rust_ptr &&
             declared_in_rule_as_rust_ptr;
    }

    bool needs_lvalue() const {
      return access == TranslationRule::Access::kWrite;
    }

    void dump() const;
  };

  std::pair<RsExpr *, std::optional<TempMaterializationCtx>>
  ConvertCallExpr(clang::CallExpr *expr);

  struct CallArg {
    enum class Kind : int8_t {
      Hoisted,
      Inline,
      Materialized,
    };

    std::string param_name;
    RsExpr *ref_temp_name = nullptr;
    clang::QualType param_type;
    clang::Expr *expr;
    bool has_default;
    Kind kind;
  };

  struct CallInfo {
    std::vector<CallArg> args;
    std::vector<clang::Expr *> variadic_args;
    clang::CallExpr *expr;
    bool is_variadic;
    bool is_fn_ptr_call;
    bool is_libc_passthrough;
  };

  CallInfo CollectCallInfo(clang::CallExpr *expr);

  RsExpr *ConvertParamTy(clang::QualType param_type, clang::Expr *expr);

  RsExpr *EmitHoistedArgs(CallInfo &info);

  RsExpr *EmitArgList(const CallInfo &info);

  RsExpr *EmitCall(CallInfo &&info);

  RsExpr *ConvertGenericCallExpr(clang::CallExpr *expr);

  virtual RsExpr *EmitFnPtrCall(clang::Expr *callee);

  virtual RsExpr *
  ConvertFunctionToFunctionPointer(const clang::FunctionDecl *fn_decl);

  // Option<fn> implements Copy
  virtual bool FunctionPointerImplementsCopy() const { return true; }

  bool TypeIsCopyable(clang::QualType ty) const {
    if (ty->isFunctionPointerType() || ty->isFunctionType()) {
      return FunctionPointerImplementsCopy();
    }
    if (ty->isBuiltinType() || ty->isEnumeralType()) {
      return true;
    }
    if (auto *record = ty->getAsRecordDecl()) {
      return RecordDerivesCopy(record);
    }
    return false;
  }

  virtual RsExpr *ConvertPrintf(clang::CallExpr *expr);

  RsExpr *ConvertVAArgCall(clang::CallExpr *expr);

  virtual RsExpr *ConvertVariadicArg(clang::Expr *arg);

  virtual RsExpr *VisitCallExpr(clang::CallExpr *expr);

  virtual RsExpr *VisitIntegerLiteral(clang::IntegerLiteral *expr);

  virtual RsExpr *VisitFloatingLiteral(clang::FloatingLiteral *expr);

  virtual RsExpr *VisitCharacterLiteral(clang::CharacterLiteral *expr);

  std::string GetEscapedCharLiteral(char character) const;

  std::string GetEscapedUTF8CharLiteral(clang::Expr *expr) const;

  std::string GetEscapedStringLiteral(clang::Expr *expr,
                                      uint64_t pad_nulls = 0) const;
  virtual RsExpr *VisitStringLiteral(clang::StringLiteral *expr);

  virtual RsExpr *VisitCXXBoolLiteralExpr(clang::CXXBoolLiteralExpr *expr);

  RsExpr *ConvertIntegerToEnumeralCast(clang::Expr *to, clang::Expr *from);

  RsExpr *ConvertIntegralToBooleanCast(clang::ImplicitCastExpr *expr);

  virtual RsExpr *VisitImplicitCastExpr(clang::ImplicitCastExpr *expr);

  virtual RsExpr *VisitExplicitCastExpr(clang::ExplicitCastExpr *expr);

  virtual RsExpr *VisitBinaryOperator(clang::BinaryOperator *expr);

  virtual RsExpr *ConvertBinaryOperator(clang::BinaryOperator *expr);

  virtual RsExpr *ConvertIncAndDec(clang::UnaryOperator *expr);

  virtual RsExpr *VisitUnaryOperator(clang::UnaryOperator *expr);

  virtual RsExpr *VisitStmtExpr(clang::StmtExpr *expr);

  virtual RsExpr *EmitStmtExprTail(clang::Expr *tail);

  virtual RsExpr *VisitConditionalOperator(clang::ConditionalOperator *expr);

  virtual RsExpr *VisitDeclRefExpr(clang::DeclRefExpr *expr);
  RsExpr *ConvertDeclRefExpr(clang::DeclRefExpr *expr);

  virtual RsExpr *VisitParenExpr(clang::ParenExpr *expr);

  RsExpr *ConvertMemberExpr(clang::MemberExpr *expr);

  virtual RsExpr *VisitMemberExpr(clang::MemberExpr *expr);

  virtual RsExpr *VisitCXXThisExpr(clang::CXXThisExpr *expr);

  virtual RsExpr *VisitInitListExpr(clang::InitListExpr *expr);

  virtual RsExpr *VisitCompoundLiteralExpr(clang::CompoundLiteralExpr *expr);

  virtual RsExpr *VisitArraySubscriptExpr(clang::ArraySubscriptExpr *expr);

  virtual RsExpr *
  VisitCXXNullPtrLiteralExpr(clang::CXXNullPtrLiteralExpr *expr);

  virtual RsExpr *VisitGNUNullExpr(clang::GNUNullExpr *expr);

  virtual RsExpr *VisitCXXNewExpr(clang::CXXNewExpr *expr);

  virtual RsExpr *VisitCXXDeleteExpr(clang::CXXDeleteExpr *expr);

  virtual RsExpr *VisitCXXConstructExpr(clang::CXXConstructExpr *expr);

  RsExpr *ConvertCXXConstructExprArgs(clang::CXXConstructExpr *expr);

  virtual RsExpr *ConvertArrayCXXConstructExpr(clang::CXXConstructExpr *expr);

  virtual RsExpr *
  VisitUnaryExprOrTypeTraitExpr(clang::UnaryExprOrTypeTraitExpr *expr);

  virtual RsExpr *VisitTypeTraitExpr(clang::TypeTraitExpr *expr);

  virtual RsExpr *VisitOffsetOfExpr(clang::OffsetOfExpr *expr);

  virtual RsExpr *VisitEnumDecl(clang::EnumDecl *decl);

  virtual RsExpr *AddFromImpl(clang::EnumDecl *decl);

  virtual RsExpr *AddIncDecImpls(clang::EnumDecl *decl);

  virtual RsExpr *VisitCXXDefaultArgExpr(clang::CXXDefaultArgExpr *expr);

  virtual RsExpr *VisitLambdaExpr(clang::LambdaExpr *expr);

  virtual RsExpr *
  VisitImplicitValueInitExpr(clang::ImplicitValueInitExpr *expr);

  virtual RsExpr *VisitSwitchStmt(clang::SwitchStmt *stmt);

  RsExpr *EmitSwitchArm(const SwitchArm &arm, bool is_default);

  RsExpr *ConvertSwitchCaseCondition(clang::SwitchCase *stmt);

  virtual RsExpr *VisitVAArgExpr(clang::VAArgExpr *expr);

  virtual RsExpr *VisitCXXDefaultInitExpr(clang::CXXDefaultInitExpr *expr);

  virtual RsExpr *VisitPredefinedExpr(clang::PredefinedExpr *expr);

  virtual RsExpr *VisitClassTemplateDecl(clang::ClassTemplateDecl *decl);

  virtual RsExpr *
  VisitCXXStdInitializerListExpr(clang::CXXStdInitializerListExpr *expr);

protected:
  RsExpr *ConvertExpr(clang::Expr *expr,
                      std::optional<clang::QualType> ict = {});

  RsExpr *DispatchExpr(clang::Expr *expr);

  RsExpr *ConvertStmt(clang::Stmt *stmt);

  RsExpr *ConvertFullStmt(clang::Stmt *stmt);

  RsExpr *ConvertDecl(clang::Decl *decl);

  RsArena arena_;

  RsExpr *Text(std::string text) {
    return arena_.New<Verbatim>(std::move(text));
  }

  RsExpr *Text(char c) { return arena_.New<Verbatim>(std::string(1, c)); }

  template <typename... Ts> RsExpr *Cat(Ts... parts) {
    return arena_.New<Concat>(std::vector<RsExpr *>{parts...});
  }

  RsExpr *Parens(RsExpr *inner, bool enabled = true) {
    return enabled ? arena_.New<Delim>('(', ')', inner) : inner;
  }

  RsExpr *Braces(RsExpr *inner, bool enabled = true) {
    return enabled ? arena_.New<Delim>('{', '}', inner) : inner;
  }

  RsExpr *Brackets(RsExpr *inner, bool enabled = true) {
    return enabled ? arena_.New<Delim>('[', ']', inner) : inner;
  }

  RsExpr *CastTo(clang::QualType qual_type) {
    return Cat(Text(keyword::kAs), Text(GetUnsafeTypeAsString(qual_type)));
  }

  // Renders a type in the current conversion state. Only for textual queries
  // on types, never for building output.
  std::string RenderType(clang::QualType qual_type);

  const clang::Expr *GetParentExpr(const clang::Expr *expr);
  bool IsSubExprOf(const clang::Expr *sub_expr, const clang::Expr *parent_expr);

  virtual RsExpr *ConvertPointeeType(clang::QualType ptr_type);

  virtual RsExpr *GetDefaultAsString(clang::QualType qual_type);

  virtual RsExpr *GetArrayDefaultAsString(clang::QualType qual_type);

  virtual RsExpr *GetDefaultAsStringFallback(clang::QualType qual_type);

  virtual RsExpr *ConvertVarDefaultInit(clang::QualType qual_type);

  virtual std::string
  GetOverloadedFunctionName(const clang::FunctionDecl *decl);

  virtual std::string GetRecordName(const clang::NamedDecl *decl) const;

  virtual std::vector<const char *>
  GetStructAttributes(const clang::RecordDecl *decl);

  virtual std::string GetUnsafeTypeAsString(clang::QualType qual_type) const;

  virtual RsExpr *ConvertVarInit(clang::QualType qual_type, clang::Expr *expr);

  virtual RsExpr *ConvertUnsignedArithOperand(clang::Expr *expr,
                                              clang::QualType type);

  virtual RsExpr *ConvertEqualsNullPtr(clang::Expr *expr);

  virtual RsExpr *ConvertPointerSubscript(clang::ArraySubscriptExpr *expr);

  virtual RsExpr *ConvertPointerOffset(clang::Expr *base, clang::Expr *idx,
                                       bool is_addition = true);

  virtual RsExpr *ConvertArraySubscript(clang::Expr *base, clang::Expr *idx,
                                        clang::QualType type);

  RsExpr *EmitFlexibleArrayElementPtr(clang::Expr *array, clang::Expr *idx,
                                      bool is_mut);

  virtual RsExpr *ConvertAssignment(clang::Expr *lhs, clang::Expr *rhs,
                                    std::string_view assign_operator);

  virtual RsExpr *ConvertFunctionParameters(clang::FunctionDecl *decl);

  virtual RsExpr *ConvertFunctionQualifiers(clang::FunctionDecl *decl);

  virtual RsExpr *ConvertFunctionReturnType(clang::FunctionDecl *decl);

  virtual RsExpr *
  ConvertFunctionMain(const clang::FunctionDecl *decl,
                      const std::string_view main_function_name);

  virtual RsExpr *ConvertAbstractClass(clang::CXXRecordDecl *decl);

  RsExpr *ConvertCXXMethodDecls(const clang::CXXRecordDecl *decl,
                                const std::string_view signature,
                                bool (*predicate)(clang::CXXMethodDecl *));

  virtual RsExpr *AddOrdTrait(const clang::CXXRecordDecl *decl);

  virtual RsExpr *
  ConvertOrdAndPartialOrdTraits(const clang::CXXRecordDecl *decl,
                                const clang::FunctionDecl *op);

  RsExpr *ConvertOrdAndPartialOrdTraitsBase(std::string_view first_branch,
                                            std::string_view second_branch,
                                            std::string_view first_return,
                                            std::string_view second_return,
                                            std::string_view record_name);

  virtual RsExpr *AddCloneTrait(const clang::RecordDecl *decl);

  virtual RsExpr *AddDropTrait(const clang::CXXRecordDecl *decl);

  virtual RsExpr *AddDefaultTrait(const clang::RecordDecl *decl);

  virtual RsExpr *AddDefaultTraitForUnion(const clang::RecordDecl *decl);

  RsExpr *EmitDefaultStructLiteral(const clang::RecordDecl *decl);

  virtual RsExpr *AddByteReprTrait(const clang::RecordDecl *decl);

  virtual RsExpr *AddByteReprTrait(const clang::EnumDecl *decl);

  virtual RsExpr *
  ConvertUnsignedArithBinaryOperator(clang::BinaryOperator *binary_operator,
                                     clang::Expr *expr);

  virtual RsExpr *ConvertAddrOf(clang::Expr *expr,
                                clang::QualType pointer_type);

  virtual RsExpr *ConvertDeref(clang::Expr *expr);

  RsExpr *EmitDeref(RsExpr *inner, clang::QualType pointee_type);

  virtual RsExpr *ConvertArrow(clang::Expr *expr);

  virtual RsExpr *ConvertLoopVariable(clang::VarDecl *decl,
                                      clang::Expr *range_init);

  virtual RsExpr *ConvertUniquePtrDeref(clang::CXXOperatorCallExpr *expr);

  virtual RsExpr *ConvertCXXOperatorCallExpr(clang::CXXOperatorCallExpr *expr);

  RsExpr *GetMappedAsNode(clang::Expr *expr, clang::Expr **args = nullptr,
                          unsigned num_args = 0,
                          TempMaterializationCtx *ctx = nullptr);

  RsExpr *
  ConvertIRFragment(const std::vector<TranslationRule::BodyFragment> &fragments,
                    clang::Expr *expr, clang::Expr **args, unsigned num_args,
                    TempMaterializationCtx *ctx);

  RsExpr *ConvertPlaceholder(clang::Expr *expr, clang::Expr *arg,
                             const PlaceholderCtx &ph_ctx);

  RsExpr *ConvertVariadicTail(clang::Expr *expr,
                              const std::vector<clang::Expr *> &all_args);

  virtual RsExpr *ConvertMappedMethodCall(
      clang::Expr *expr, const TranslationRule::MethodCallFragment &mc,
      clang::Expr **args, unsigned num_args, TempMaterializationCtx *ctx);

  virtual RsExpr *AccessLValueObject(clang::MemberExpr *member);

  virtual RsExpr *ConvertGenericBinaryOperator(clang::BinaryOperator *expr);

  virtual bool IsReferenceType(const clang::Expr *expr) const;

  virtual bool RecordDerivesDefault(const clang::RecordDecl *decl);

  bool RecordDerivesCopy(const clang::RecordDecl *decl) const;

  bool RecordHasCopyableFields(const clang::RecordDecl *decl);

  bool ShouldReplaceWithMappedBody(clang::DeclRefExpr *expr) const;

  std::string *rs_code_;
  clang::ASTContext &ctx_;
  clang::FunctionDecl *curr_function_ = nullptr;
  bool in_function_formals_ = false;
  bool in_const_initializer_ = false;
  std::optional<bool> autoref_mut_;
  bool suppress_iterator_clone_ = false;

  struct PushExplicitAutoref {
    Converter &c;
    std::optional<bool> prev;
    PushExplicitAutoref(Converter &c, std::optional<bool> v)
        : c(c), prev(c.autoref_mut_) {
      c.autoref_mut_ = v;
    }
    ~PushExplicitAutoref() { c.autoref_mut_ = prev; }
  };

  struct PushSuppressIteratorClone {
    Converter &c;
    bool prev;
    PushSuppressIteratorClone(Converter &c, clang::CXXConstructExpr *expr)
        : c(c), prev(c.suppress_iterator_clone_) {
      auto *ctor = expr->getConstructor();
      if (!ctor->isCopyOrMoveConstructor() &&
          ctor->isConvertingConstructor(/*AllowExplicit=*/false) &&
          ctor->getNumParams() == 1 && IsIteratorType(expr->getType())) {
        c.suppress_iterator_clone_ = true;
      }
    }
    ~PushSuppressIteratorClone() { c.suppress_iterator_clone_ = prev; }
    PushSuppressIteratorClone(const PushSuppressIteratorClone &) = delete;
    PushSuppressIteratorClone &
    operator=(const PushSuppressIteratorClone &) = delete;

    static bool take(Converter &c) {
      return std::exchange(c.suppress_iterator_clone_, false);
    }

  private:
    static bool IsIteratorType(clang::QualType qt) {
      if (auto *record = qt->getAsCXXRecordDecl()) {
        for (auto *d : record->decls()) {
          if (auto *tnd = llvm::dyn_cast<clang::TypedefNameDecl>(d)) {
            if (tnd->getName() == "iterator_category")
              return true;
          }
        }
      }
      return false;
    }
  };

  struct PushConstInitializer {
    Converter &c;
    bool prev;
    bool enabled;
    PushConstInitializer(Converter &c, bool enabled)
        : c(c), prev(c.in_const_initializer_), enabled(enabled) {
      if (enabled) {
        c.in_const_initializer_ = true;
      }
    }
    ~PushConstInitializer() {
      if (enabled) {
        c.in_const_initializer_ = prev;
      }
    }
  };
  std::vector<clang::Expr *> curr_for_inc_;
  std::vector<clang::QualType> curr_init_type_;

  enum class BreakTarget : int8_t { Loop, FallthroughSwitch, Switch };
  std::vector<BreakTarget> break_target_;

  bool isSwitchBreak() const {
    return !break_target_.empty() &&
           break_target_.back() == BreakTarget::Switch;
  }

  class PushBreakTarget {
  public:
    PushBreakTarget(std::vector<BreakTarget> &stack, BreakTarget target)
        : stack_(stack) {
      stack_.push_back(target);
    }
    ~PushBreakTarget() { stack_.pop_back(); }
    PushBreakTarget(const PushBreakTarget &) = delete;
    PushBreakTarget &operator=(const PushBreakTarget &) = delete;

  private:
    std::vector<BreakTarget> &stack_;
  };

  class PushInitType {
  public:
    PushInitType(Converter &c, clang::QualType type) : c_(c) {
      c_.curr_init_type_.emplace_back(type);
    }
    ~PushInitType() { c_.curr_init_type_.pop_back(); }
    PushInitType(const PushInitType &) = delete;
    PushInitType &operator=(const PushInitType &) = delete;

  private:
    Converter &c_;
  };

  std::unordered_set<const clang::VarDecl *> map_iter_decls_;

  // Local variables hoisted outside a goto_block so that all labels can see and
  // use the variables.
  std::unordered_set<const clang::VarDecl *> hoisted_decls_;
  class PushHoistedDecls {
  public:
    PushHoistedDecls(std::unordered_set<const clang::VarDecl *> &field)
        : field_(field), saved_(std::move(field)) {
      field_.clear();
    }
    ~PushHoistedDecls() { field_ = std::move(saved_); }
    PushHoistedDecls(const PushHoistedDecls &) = delete;
    PushHoistedDecls &operator=(const PushHoistedDecls &) = delete;

  private:
    std::unordered_set<const clang::VarDecl *> &field_;
    std::unordered_set<const clang::VarDecl *> saved_;
  };

  struct ScopedMapIterDecl {
    Converter &c;
    const clang::VarDecl *decl;
    ScopedMapIterDecl(Converter &c, const clang::VarDecl *decl)
        : c(c), decl(decl) {
      c.map_iter_decls_.insert(decl);
    }
    ~ScopedMapIterDecl() { c.map_iter_decls_.erase(decl); }
  };
  static std::unordered_set<std::string> decl_ids_;
  static std::unordered_set<std::string> abstract_structs_;

  class RecordIndex {
  public:
    void MarkReferenced(std::string name) {
      entries_.try_emplace(std::move(name), false);
    }
    // Returns false if `name` is already defined; otherwise marks it and
    // returns true.
    bool MarkDefined(const std::string &name) {
      bool &defined = entries_[name];
      if (defined) {
        return false;
      }
      defined = true;
      return true;
    }
    template <typename F> void ForEachUndefined(F &&f) const {
      for (const auto &[name, defined] : entries_) {
        if (!defined) {
          f(name);
        }
      }
    }

  private:
    // record name -> true if a definition has been emitted, false if only
    // referenced.
    std::unordered_map<std::string, bool> entries_;
  };
  static RecordIndex record_decls_;

  enum class ExprKind : uint8_t {
    Callee,
    LValue,
    RValue,
    XValue,
    AddrOf,
    Object,
    Void,
  };

  static const char *expr_kind_to_string(ExprKind kind) {
    switch (kind) {
    case ExprKind::Callee:
      return "Callee";
    case ExprKind::LValue:
      return "LValue";
    case ExprKind::RValue:
      return "RValue";
    case ExprKind::XValue:
      return "XValue";
    case ExprKind::AddrOf:
      return "AddrOf";
    case ExprKind::Object:
      return "Object";
    case ExprKind::Void:
      return "Void";
    default:
      return "Unknown";
    }
  }

  bool isLValue() const;
  bool isRValue() const;
  bool isXValue() const;
  bool isAddrOf() const;
  bool isObject() const;
  bool isVoid() const;
  bool isCallee() const;

  void dump_expr_kinds();

  struct PushExprKind {
    Converter &c;
    PushExprKind(Converter &c, ExprKind k, const char *file = __builtin_FILE(),
                 int line = __builtin_LINE())
        : c(c) {
      c.curr_expr_kind_.push_back(k);
      log() << "PushExprKind " << file << ':' << line << ' ';
      c.dump_expr_kinds();
      log() << '[';
      for (const auto k : c.curr_expr_kind_) {
        log() << c.expr_kind_to_string(k) << ", ";
      }
      log() << "]\n";
    }
    ~PushExprKind() { c.curr_expr_kind_.pop_back(); }
  };

  enum class ComputedExprType : uint8_t {
    Value,
    FreshValue,
    Pointer,
    FreshPointer,
  };
  ComputedExprType computed_expr_type_ = ComputedExprType::FreshValue;

  bool isFresh() const {
    return computed_expr_type_ == ComputedExprType::FreshValue ||
           computed_expr_type_ == ComputedExprType::FreshPointer;
  }

  void SetFresh();
  void SetValueFreshness(clang::QualType type);
  void SetFreshType(clang::QualType type);

  RsExpr *ConvertLValue(clang::Expr *expr);
  RsExpr *ConvertRValue(clang::Expr *expr,
                        std::optional<clang::QualType> implicit_convert_to = {},
                        int line = __builtin_LINE());
  virtual RsExpr *
  ConvertFreshRValue(clang::Expr *expr,
                     std::optional<clang::QualType> implicit_convert_to = {});
  virtual RsExpr *ConvertFreshPointer(clang::Expr *expr);
  virtual RsExpr *ConvertFreshObject(clang::Expr *expr);
  RsExpr *ConvertPointer(clang::Expr *expr, int line = __builtin_LINE());

  /// Materialize a temporary for a prvalue bound to a reference parameter.
  /// Returns (binding_code, ref_expression).
  virtual std::pair<RsExpr *, RsExpr *>
  MaterializeTemp(const std::string &binding_name, clang::QualType param_type,
                  clang::Expr *expr);

  // TODO: move this into the Plugin infrastructure. Plugins are used for
  // functions that cannot be translated using the rules/ directory. For
  // example emplace_back, make_unique, printf, etc. Generally variadic
  // argument functions and functions that use perfect forwarding.
  RsExpr *TryPluginConvert(clang::CallExpr *call);

  bool emplace_back_plugin_match(clang::CallExpr *call);
  virtual RsExpr *emplace_back_plugin_convert(clang::CallExpr *call);
  virtual RsExpr *
  emplace_back_plugin_construct_arg(clang::QualType elem_type,
                                    clang::CXXConstructExpr *ctor);
  virtual RsExpr *emplace_back_emit_push_open(clang::CXXMemberCallExpr *call);
  virtual RsExpr *emplace_back_emit_push_close(clang::CXXMemberCallExpr *call);

  virtual const char *GetPointerDerefPrefix(clang::QualType pointee_type);

  TempMaterializationCtx CollectRefBindingTempArgs(clang::CallExpr *expr);

  bool IsCastRedundantInRust(clang::Expr *expr, clang::QualType target_type);

private:
  void materializeTemplateSpecialization(clang::CXXRecordDecl *decl);

  std::string getIntegerLiteral(clang::IntegerLiteral *expr, bool incl_type,
                                const clang::QualType *type = nullptr);
  const char *keyword_unsafe_;
  const char *keyword_mut_;
  const char *keyword_const_fn_;
  std::vector<ExprKind> curr_expr_kind_;
  static std::unordered_map<std::string, std::string> inner_structs_;
  static std::unordered_set<std::string> globals_;
  clang::Sema *sema_ = nullptr;
};
} // namespace cpp2rust
