#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/RecursiveASTVisitor.h>
#include <clang/Sema/Sema.h>

#include <functional>
#include <optional>
#include <string>
#include <type_traits>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

#include "converter/converter_lib.h"
#include "converter/lex.h"
#include "converter/translation_rule.h"
#include "logging.h"

namespace cpp2rust {
class Converter : public clang::RecursiveASTVisitor<Converter> {

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

  bool VisitRecoveryExpr(clang::RecoveryExpr *expr);

  virtual void EmitFilePreamble();

  static std::string EmitOpaqueRecords();

  virtual bool VisitBuiltinType(clang::BuiltinType *type);

  virtual bool VisitRecordType(clang::RecordType *type);

  virtual bool VisitConstantArrayType(clang::ConstantArrayType *type);

  virtual bool VisitIncompleteArrayType(clang::IncompleteArrayType *type);

  virtual bool VisitLValueReferenceType(clang::LValueReferenceType *type);

  virtual bool VisitPointerType(clang::PointerType *type);

  enum class FnProtoType { LambdaCallOperator, FnPtr };

  virtual std::string
  ConvertFunctionPointerType(const clang::FunctionProtoType *proto,
                             FnProtoType kind = FnProtoType::FnPtr);

  virtual bool VisitDecayedType(clang::DecayedType *type);

  virtual bool VisitTypedefType(clang::TypedefType *type);

  virtual bool VisitUsingType(clang::UsingType *type);

  virtual bool VisitTranslationUnitDecl(clang::TranslationUnitDecl *decl);

  virtual bool VisitFunctionDecl(clang::FunctionDecl *decl);

  virtual void EmitFunctionPreamble(clang::FunctionDecl *decl);

  virtual void ConvertFunctionBody(clang::FunctionDecl *decl);

  void ConvertGotoBlock(clang::CompoundStmt *body);

  void EmitHoistedDecls(clang::CompoundStmt *body);

  virtual bool VisitFunctionTemplateDecl(clang::FunctionTemplateDecl *decl);

  virtual bool VisitVarDecl(clang::VarDecl *decl);

  void ConvertVarDecl(clang::VarDecl *decl);

  virtual void EmitHoistedInArmAssignment(clang::VarDecl *decl);

  void ConvertVarDeclInitializer(clang::VarDecl *decl);

  virtual void ConvertGlobalVarDecl(clang::VarDecl *decl);

  virtual void ConvertVaListVarDecl(clang::VarDecl *decl);

  virtual bool ConvertVarDeclSkipInit(clang::VarDecl *decl);

  virtual bool ConvertLambdaVarDecl(clang::VarDecl *decl);

  bool VisitRecordDecl(clang::RecordDecl *decl);

  virtual bool VisitCXXRecordDecl(clang::CXXRecordDecl *decl);

  virtual void EmitRustStructOrUnion(clang::RecordDecl *decl);

  virtual bool EmitsReprCForRecords() const { return true; }

  virtual bool VisitCXXMethodDecl(clang::CXXMethodDecl *decl);
  virtual std::string GetSelfMaybeWithMut(const clang::CXXMethodDecl *decl);

  void ConvertCXXConstructorBody(clang::CXXConstructorDecl *decl);

  virtual bool VisitCXXConstructorDecl(clang::CXXConstructorDecl *decl);

  virtual bool VisitFieldDecl(clang::FieldDecl *decl);

  virtual bool VisitNamespaceDecl(clang::NamespaceDecl *decl);

  virtual bool VisitTypedefDecl(clang::TypedefDecl *decl);

  virtual bool VisitCompoundStmt(clang::CompoundStmt *stmt);

  virtual bool VisitDeclStmt(clang::DeclStmt *stmt);

  virtual bool VisitReturnStmt(clang::ReturnStmt *stmt);

  virtual bool VisitGotoStmt(clang::GotoStmt *stmt);

  void ConvertCondition(clang::Expr *cond);

  virtual bool VisitIfStmt(clang::IfStmt *stmt);

  virtual bool VisitWhileStmt(clang::WhileStmt *stmt);

  virtual bool VisitDoStmt(clang::DoStmt *stmt);

  virtual bool VisitForStmt(clang::ForStmt *stmt);

  virtual bool VisitCXXForRangeStmt(clang::CXXForRangeStmt *stmt);

  virtual bool VisitCXXForRangeStmtMap(clang::CXXForRangeStmt *stmt);

  virtual bool VisitCXXForRangeStmtVector(clang::CXXForRangeStmt *stmt);

  virtual bool VisitCXXForRangeStmtString(clang::CXXForRangeStmt *stmt);

  bool VisitCXXForRangeStmtIndexBased(clang::CXXForRangeStmt *stmt,
                                      const char *len_suffix);

  void ConvertForRangeBody(clang::CXXForRangeStmt *stmt,
                           const clang::VarDecl *map_iter_decl = nullptr);

  virtual bool VisitBreakStmt(clang::BreakStmt *stmt);

  virtual bool VisitContinueStmt(clang::ContinueStmt *stmt);

  bool GetFmtArg(clang::Expr *arg, std::string &fmt, std::string &fmt_args,
                 const char *&fmt_trait, std::string &fmt_width);

  bool GetRawArg(clang::Expr *arg, std::string &raw_args);

  void ConvertCallToOstream(clang::CallExpr *expr);
  virtual std::string ConvertStream(clang::Expr *expr);

  struct TempMaterializationCtx {
    std::vector<std::optional<clang::QualType>> materialized_args;
    std::string temporary_bindings;

    TempMaterializationCtx(size_t num_args)
        : materialized_args(num_args), materialized_refs_(num_args) {}

    const std::string &GetOrMaterialize(
        unsigned argument_num,
        std::function<std::pair<std::string, std::string>(const std::string &,
                                                          clang::QualType)>
            materialize_fn);

  private:
    std::vector<std::string> materialized_refs_;
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

  std::optional<TempMaterializationCtx> ConvertCallExpr(clang::CallExpr *expr);

  struct CallArg {
    enum class Kind : int8_t {
      Hoisted,
      Inline,
      Materialized,
    };

    std::string param_name;
    std::string ref_temp_name;
    clang::QualType param_type;
    clang::Expr *expr;
    bool has_default;
    Kind kind;
  };

  struct CallInfo {
    std::vector<CallArg> args;
    std::vector<clang::Expr *> variadic_args;
    clang::Expr *callee;
    bool is_variadic;
    bool is_fn_ptr_call;
  };

  CallInfo CollectCallInfo(clang::CallExpr *expr);

  void ConvertParamTy(clang::QualType param_type, clang::Expr *expr);

  void EmitHoistedArgs(CallInfo &info);

  void EmitArgList(const CallInfo &info);

  void EmitCall(CallInfo &&info);

  void ConvertGenericCallExpr(clang::CallExpr *expr);

  virtual void EmitFnPtrCall(clang::Expr *callee);

  virtual void
  ConvertFunctionToFunctionPointer(const clang::FunctionDecl *fn_decl);

  // Option<fn> implements Copy
  virtual bool FunctionPointerImplementsCopy() const { return true; }

  virtual void ConvertPrintf(clang::CallExpr *expr);

  void ConvertVAArgCall(clang::CallExpr *expr);

  virtual void ConvertVariadicArg(clang::Expr *arg);

  virtual bool VisitCallExpr(clang::CallExpr *expr);

  virtual bool VisitIntegerLiteral(clang::IntegerLiteral *expr);

  virtual bool VisitFloatingLiteral(clang::FloatingLiteral *expr);

  virtual bool VisitCharacterLiteral(clang::CharacterLiteral *expr);

  std::string GetEscapedCharLiteral(char character) const;

  std::string GetEscapedUTF8CharLiteral(clang::Expr *expr) const;

  std::string GetEscapedStringLiteral(clang::Expr *expr,
                                      uint64_t pad_nulls = 0) const;
  virtual bool VisitStringLiteral(clang::StringLiteral *expr);

  virtual bool VisitCXXBoolLiteralExpr(clang::CXXBoolLiteralExpr *expr);

  void ConvertIntegerToEnumeralCast(clang::Expr *to, clang::Expr *from);

  void ConvertIntegralToBooleanCast(clang::ImplicitCastExpr *expr);

  virtual bool VisitImplicitCastExpr(clang::ImplicitCastExpr *expr);

  virtual bool VisitExplicitCastExpr(clang::ExplicitCastExpr *expr);

  virtual bool VisitBinaryOperator(clang::BinaryOperator *expr);

  virtual void ConvertBinaryOperator(clang::BinaryOperator *expr);

  virtual bool ConvertIncAndDec(clang::UnaryOperator *expr);

  virtual bool VisitUnaryOperator(clang::UnaryOperator *expr);

  virtual bool VisitStmtExpr(clang::StmtExpr *expr);

  virtual void EmitStmtExprTail(clang::Expr *tail);

  virtual bool VisitConditionalOperator(clang::ConditionalOperator *expr);

  virtual bool VisitDeclRefExpr(clang::DeclRefExpr *expr);
  std::string ConvertDeclRefExpr(clang::DeclRefExpr *expr);

  virtual bool VisitParenExpr(clang::ParenExpr *expr);

  void ConvertMemberExpr(clang::MemberExpr *expr);

  virtual bool VisitMemberExpr(clang::MemberExpr *expr);

  virtual bool VisitCXXThisExpr(clang::CXXThisExpr *expr);

  virtual bool VisitInitListExpr(clang::InitListExpr *expr);

  virtual bool VisitCompoundLiteralExpr(clang::CompoundLiteralExpr *expr);

  virtual bool VisitArraySubscriptExpr(clang::ArraySubscriptExpr *expr);

  virtual bool VisitCXXNullPtrLiteralExpr(clang::CXXNullPtrLiteralExpr *expr);

  virtual bool VisitGNUNullExpr(clang::GNUNullExpr *expr);

  virtual bool VisitCXXNewExpr(clang::CXXNewExpr *expr);

  virtual bool VisitCXXDeleteExpr(clang::CXXDeleteExpr *expr);

  virtual bool VisitCXXConstructExpr(clang::CXXConstructExpr *expr);

  void ConvertCXXConstructExprArgs(clang::CXXConstructExpr *expr);

  virtual void ConvertArrayCXXConstructExpr(clang::CXXConstructExpr *expr);

  virtual bool
  VisitUnaryExprOrTypeTraitExpr(clang::UnaryExprOrTypeTraitExpr *expr);

  virtual bool VisitTypeTraitExpr(clang::TypeTraitExpr *expr);

  virtual bool VisitOffsetOfExpr(clang::OffsetOfExpr *expr);

  virtual bool VisitEnumDecl(clang::EnumDecl *decl);

  virtual void AddFromImpl(clang::EnumDecl *decl);

  virtual void AddIncDecImpls(clang::EnumDecl *decl);

  virtual bool VisitCXXDefaultArgExpr(clang::CXXDefaultArgExpr *expr);

  virtual bool VisitLambdaExpr(clang::LambdaExpr *expr);

  virtual bool VisitImplicitValueInitExpr(clang::ImplicitValueInitExpr *expr);

  virtual bool VisitSwitchStmt(clang::SwitchStmt *stmt);

  void EmitSwitchArm(const SwitchArm &arm, bool is_default);

  bool ConvertSwitchCaseCondition(clang::SwitchCase *stmt);

  virtual bool VisitVAArgExpr(clang::VAArgExpr *expr);

  virtual bool VisitCXXDefaultInitExpr(clang::CXXDefaultInitExpr *expr);

  virtual bool VisitPredefinedExpr(clang::PredefinedExpr *expr);

  virtual bool VisitClassTemplateDecl(clang::ClassTemplateDecl *decl);

  virtual bool
  VisitCXXStdInitializerListExpr(clang::CXXStdInitializerListExpr *expr);

protected:
  const clang::Expr *GetParentExpr(const clang::Expr *expr);
  bool IsSubExprOf(const clang::Expr *sub_expr, const clang::Expr *parent_expr);

#define StrCat(...) _StrCat(__FUNCTION__, __LINE__, __VA_ARGS__)

  template <typename... Ts>
  inline void _StrCat(const char *func, int line, const Ts &...vals) {
    log() << '[' << func << ':' << line << "] ";
    ((log() << vals << '\n', *rs_code_ += vals, *rs_code_ += ' '), ...);
  }

  class Buffer {
    std::string partial_code;
    std::string *full_code;
    Converter &c;

  public:
    Buffer(Converter &c) : full_code(c.rs_code_), c(c) {
      c.rs_code_ = &partial_code;
    }
    ~Buffer() { c.rs_code_ = full_code; }
    std::string str() && { return std::move(partial_code); }
  };

  template <char kOpen, char kClose> class PushDelim {
    Converter &c;
    bool enabled;

  public:
    PushDelim(Converter &c, bool enabled = true) : c(c), enabled(enabled) {
      if (enabled) {
        c.StrCat(kOpen);
      }
    }
    ~PushDelim() {
      if (enabled) {
        c.StrCat(kClose);
      }
    }
    PushDelim(const PushDelim &) = delete;
    PushDelim(PushDelim &&) = delete;
    PushDelim &operator=(const PushDelim &) = delete;
    PushDelim &operator=(PushDelim &&) = delete;
  };

  using PushBrace =
      PushDelim<token::kOpenCurlyBracket, token::kCloseCurlyBracket>;
  using PushParen = PushDelim<token::kOpenParen, token::kCloseParen>;
  using PushBracket = PushDelim<token::kOpenBracket, token::kCloseBracket>;

  template <typename T>
  inline std::string
  ToString(T node, std::optional<clang::QualType> implicit_convert_to = {}) {
    Buffer buf(*this);
    if constexpr (std::is_convertible_v<T, clang::Expr *>) {
      Convert(node, implicit_convert_to);
    } else {
      Convert(node);
    }
    return std::move(buf).str();
  }

  template <typename T> inline std::string ToStringBase(T node) {
    Buffer buf(*this);
    Converter::Convert(node);
    return std::move(buf).str();
  }

  virtual bool Convert(clang::QualType qual_type);
  virtual bool ConvertMappedType(clang::QualType qual_type);

  virtual std::string ConvertPointeeType(clang::QualType ptr_type);

  virtual bool Convert(clang::Decl *decl);
  virtual bool Convert(clang::Stmt *stmt);
  virtual bool Convert(clang::Expr *expr,
                       std::optional<clang::QualType> implicit_convert_to = {});

  virtual std::string GetDefaultAsString(clang::QualType qual_type);

  virtual std::string GetArrayDefaultAsString(clang::QualType qual_type);

  virtual std::string GetDefaultAsStringFallback(clang::QualType qual_type);

  virtual std::string ConvertVarDefaultInit(clang::QualType qual_type);

  virtual std::string
  GetOverloadedFunctionName(const clang::FunctionDecl *decl);

  virtual std::string GetRecordName(const clang::NamedDecl *decl) const;

  virtual std::vector<const char *>
  GetStructAttributes(const clang::RecordDecl *decl);

  virtual std::string GetUnsafeTypeAsString(clang::QualType qual_type) const;

  virtual void ConvertVarInit(clang::QualType qual_type, clang::Expr *expr);

  virtual void ConvertUnsignedArithOperand(clang::Expr *expr,
                                           clang::QualType type);

  virtual void ConvertEqualsNullPtr(clang::Expr *expr);

  virtual void ConvertPointerSubscript(clang::ArraySubscriptExpr *expr);

  virtual void ConvertPointerOffset(clang::Expr *base, clang::Expr *idx,
                                    bool is_addition = true);

  virtual void ConvertArraySubscript(clang::Expr *base, clang::Expr *idx,
                                     clang::QualType type);

  void EmitFlexibleArrayElementPtr(clang::Expr *array, clang::Expr *idx,
                                   bool is_mut);

  virtual void ConvertAssignment(clang::Expr *lhs, clang::Expr *rhs,
                                 std::string_view assign_operator);

  virtual void ConvertFunctionParameters(clang::FunctionDecl *decl);

  virtual void ConvertFunctionQualifiers(clang::FunctionDecl *decl);

  virtual void ConvertFunctionReturnType(clang::FunctionDecl *decl);

  virtual void ConvertFunctionMain(const clang::FunctionDecl *decl,
                                   const std::string_view main_function_name);

  virtual void ConvertAbstractClass(clang::CXXRecordDecl *decl);

  void ConvertCXXMethodDecls(const clang::CXXRecordDecl *decl,
                             const std::string_view signature,
                             bool (*predicate)(clang::CXXMethodDecl *));

  virtual void AddOrdTrait(const clang::CXXRecordDecl *decl);

  virtual void ConvertOrdAndPartialOrdTraits(const clang::CXXRecordDecl *decl,
                                             const clang::FunctionDecl *op);

  void ConvertOrdAndPartialOrdTraitsBase(std::string_view first_branch,
                                         std::string_view second_branch,
                                         std::string_view first_return,
                                         std::string_view second_return,
                                         std::string_view record_name);

  virtual void AddCloneTrait(const clang::CXXRecordDecl *decl);

  virtual void AddDropTrait(const clang::CXXRecordDecl *decl);

  virtual void AddDefaultTrait(const clang::RecordDecl *decl);

  virtual void AddDefaultTraitForUnion(const clang::RecordDecl *decl);

  void EmitDefaultStructLiteral(const clang::RecordDecl *decl);

  virtual void AddByteReprTrait(const clang::RecordDecl *decl);

  virtual void
  ConvertUnsignedArithBinaryOperator(clang::BinaryOperator *binary_operator,
                                     clang::Expr *expr);

  virtual void ConvertAddrOf(clang::Expr *expr, clang::QualType pointer_type);

  virtual void ConvertDeref(clang::Expr *expr);

  void EmitDeref(std::string inner, clang::QualType pointee_type);

  virtual void ConvertArrow(clang::Expr *expr);

  virtual void ConvertCast(clang::QualType qual_type,
                           int line = __builtin_LINE());

  virtual void ConvertLoopVariable(clang::VarDecl *decl,
                                   clang::Expr *range_init);

  virtual void ConvertUniquePtrDeref(clang::CXXOperatorCallExpr *expr);

  virtual bool ConvertCXXOperatorCallExpr(clang::CXXOperatorCallExpr *expr);

  std::string GetMappedAsString(clang::Expr *expr, clang::Expr **args = nullptr,
                                unsigned num_args = 0,
                                TempMaterializationCtx *ctx = nullptr);

  std::string
  ConvertIRFragment(const std::vector<TranslationRule::BodyFragment> &fragments,
                    clang::Expr *expr, clang::Expr **args, unsigned num_args,
                    TempMaterializationCtx *ctx);

  std::string ConvertPlaceholder(clang::Expr *expr, clang::Expr *arg,
                                 const PlaceholderCtx &ph_ctx);

  virtual std::string ConvertMappedMethodCall(
      clang::Expr *expr, const TranslationRule::MethodCallFragment &mc,
      clang::Expr **args, unsigned num_args, TempMaterializationCtx *ctx);

  virtual std::string AccessLValueObject(clang::MemberExpr *member);

  virtual void ConvertGenericBinaryOperator(clang::BinaryOperator *expr);

  virtual bool IsReferenceType(const clang::Expr *expr) const;

  virtual bool RecordDerivesDefault(const clang::RecordDecl *decl);

  bool RecordDerivesCopy(const clang::RecordDecl *decl);

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

  std::string ConvertLValue(clang::Expr *expr);
  std::string
  ConvertRValue(clang::Expr *expr,
                std::optional<clang::QualType> implicit_convert_to = {},
                int line = __builtin_LINE());
  virtual std::string
  ConvertFreshRValue(clang::Expr *expr,
                     std::optional<clang::QualType> implicit_convert_to = {});
  virtual std::string ConvertFreshPointer(clang::Expr *expr);
  virtual std::string ConvertFreshObject(clang::Expr *expr);
  std::string ConvertPointer(clang::Expr *expr, int line = __builtin_LINE());

  /// Materialize a temporary for a prvalue bound to a reference parameter.
  /// Returns (binding_code, ref_expression).
  virtual std::pair<std::string, std::string>
  MaterializeTemp(const std::string &binding_name, clang::QualType param_type,
                  clang::Expr *expr);

  // TODO: move this into the Plugin infrastructure. Plugins are used for
  // functions that cannot be translated using the rules/ directory. For
  // example emplace_back, make_unique, printf, etc. Generally variadic
  // argument functions and functions that use perfect forwarding.
  std::optional<std::string> TryPluginConvert(clang::CallExpr *call);

  bool emplace_back_plugin_match(clang::CallExpr *call);
  virtual bool emplace_back_plugin_convert(clang::CallExpr *call);
  virtual void emplace_back_plugin_construct_arg(clang::QualType elem_type,
                                                 clang::CXXConstructExpr *ctor);
  virtual void emplace_back_emit_push_open(clang::CXXMemberCallExpr *call);
  virtual void emplace_back_emit_push_close(clang::CXXMemberCallExpr *call);

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
