#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Mangle.h>
#include <clang/AST/RecursiveASTVisitor.h>
#include <clang/Sema/Sema.h>

#include <format>
#include <functional>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <type_traits>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

#include "converter/converter_lib.h"
#include "converter/factory.h"
#include "converter/lex.h"
#include "converter/translation_rule.h"
#include "logging.h"

namespace cpp2rust {
inline constexpr const char kDestructorName[] = "destructor";

// Sets var to value while in scope.
template <typename T> class ScopedValue {
  T &var_;
  T saved_;

public:
  ScopedValue(T &var, std::type_identity_t<T> value)
      : var_(var), saved_(std::exchange(var, std::move(value))) {}
  ~ScopedValue() { var_ = std::move(saved_); }
  ScopedValue(const ScopedValue &) = delete;
  ScopedValue &operator=(const ScopedValue &) = delete;
};

// Pushes value onto stack while in scope.
template <typename T> class ScopedPush {
  std::vector<T> &stack_;

public:
  ScopedPush(std::vector<T> &stack, std::type_identity_t<T> value)
      : stack_(stack) {
    stack_.push_back(std::move(value));
  }
  ~ScopedPush() { stack_.pop_back(); }
  ScopedPush(const ScopedPush &) = delete;
  ScopedPush &operator=(const ScopedPush &) = delete;
};

class Converter : public clang::RecursiveASTVisitor<Converter> {

public:
  explicit Converter(std::string &rs_code, clang::ASTContext &ctx,
                     const char *keyword_unsafe = "unsafe",
                     const char *keyword_mut = keyword::kMut)
      : rs_code_(&rs_code), ctx_(ctx), keyword_unsafe_(keyword_unsafe),
        keyword_mut_(keyword_mut) {
    if (!GetMangleContext()) {
      mangle_context_.reset(ctx.createMangleContext());
      SetMangleContext(mangle_context_.get());
    }
  }

  virtual ~Converter() {
    if (mangle_context_) {
      SetMangleContext(nullptr);
    }
  }

  Converter(const Converter &) = delete;
  Converter &operator=(const Converter &) = delete;
  Converter(Converter &&) = delete;
  Converter &operator=(Converter &&) = delete;

  void SetSema(clang::Sema &sema) { sema_ = &sema; }

  auto &GetSema() {
    assert(sema_ && "sema_ should already be set");
    return *sema_;
  }

  clang::ASTContext &GetASTContext() { return ctx_; }

  virtual std::string GetUnboxedTypeAsString(clang::QualType qual_type);

  virtual std::string GetUnboxedDefaultAsString(clang::QualType qual_type);

  std::string GetTemplateArgumentAsString(const clang::TemplateArgument &arg);

  bool VisitRecoveryExpr(clang::RecoveryExpr *expr);

  virtual void EmitFilePreamble();

  static void EmitOpaqueRecords(Model model, std::string &out);
  static void EmitGlobalInits(Model model, std::string &out);

  static void EmitVirtualMethods(std::string &out);

  virtual bool VisitBuiltinType(clang::BuiltinType *type);

  virtual bool VisitRecordType(clang::RecordType *type);

  bool VisitEnumType(clang::EnumType *type);

  virtual bool VisitConstantArrayType(clang::ConstantArrayType *type);

  virtual bool VisitIncompleteArrayType(clang::IncompleteArrayType *type);

  virtual bool VisitReferenceType(clang::ReferenceType *type);

  virtual bool VisitPointerType(clang::PointerType *type);

  virtual std::string
  ConvertFunctionPointerType(const clang::FunctionProtoType *proto);

  virtual bool VisitDecayedType(clang::DecayedType *type);

  virtual bool VisitTypedefType(clang::TypedefType *type);

  virtual bool VisitUsingType(clang::UsingType *type);

  virtual bool VisitTranslationUnitDecl(clang::TranslationUnitDecl *decl);

  virtual bool VisitFunctionDecl(clang::FunctionDecl *decl);

  virtual void EmitFunctionPreamble(clang::FunctionDecl *decl);

  // Declares the storage of the temporary temp that the default argument of
  // param is bound to, and returns its name.
  std::string
  EmitDefaultArgStorage(const clang::ParmVarDecl *param,
                        const clang::MaterializeTemporaryExpr *temp);

  virtual void ConvertFunctionBody(clang::FunctionDecl *decl);

  void ConvertGotoBlock(clang::CompoundStmt *body);

  void ConvertBody(clang::Stmt *body);

  void ConvertBodyStmts(clang::Stmt *body);

  void EmitHoistedDecls(clang::CompoundStmt *body);

  // Declares decl ahead of its initialization, with a default value.
  void EmitHoistedDecl(clang::VarDecl *decl);

  virtual bool VisitFunctionTemplateDecl(clang::FunctionTemplateDecl *decl);
  bool VisitVarTemplateDecl(clang::VarTemplateDecl *decl);

  virtual bool VisitVarDecl(clang::VarDecl *decl);
  virtual bool LazyStaticInit() const { return true; }
  virtual std::string ForceGlobalInit(const clang::VarDecl *decl);

  void ConvertVarDecl(clang::VarDecl *decl);

  virtual void EmitHoistedInArmAssignment(clang::VarDecl *decl);

  void ConvertVarDeclInitializer(clang::VarDecl *decl);

  virtual void ConvertGlobalVarDecl(clang::VarDecl *decl);

  virtual bool ConvertVarDeclSkipInit(clang::VarDecl *decl);

  bool VisitRecordDecl(clang::RecordDecl *decl);

  virtual bool VisitCXXRecordDecl(clang::CXXRecordDecl *decl);

  virtual void EmitRustStructOrUnion(clang::RecordDecl *decl);

  void EmitReprC(clang::RecordDecl *decl);
  void EmitDerives(const clang::RecordDecl *decl);

  void EmitConversionAttrs(clang::CXXRecordDecl *decl);

  virtual void EmitRustUnion(clang::RecordDecl *decl);

  virtual bool EmitsReprCForRecords() const { return true; }

  // CharRustType and CharRustTypeIsSigned should disappear after
  // https://github.com/Cpp2Rust/cpp2rust/issues/246
  virtual const char *CharRustType() const { return "libc::c_char"; }

  virtual bool CharRustTypeIsSigned() const { return true; }

  virtual bool VisitCXXMethodDecl(clang::CXXMethodDecl *decl);

  virtual bool ShouldConvertMethod(const clang::CXXMethodDecl *decl);

  virtual bool ConvertOutOfLineMethod(clang::CXXMethodDecl *decl);

  bool ConvertCXXMethodDecl(clang::CXXMethodDecl *decl);

  std::string GetMethodName(const clang::CXXMethodDecl *decl);

  std::string GetConversionName(const clang::CXXConversionDecl *decl) const;

  virtual std::string GetSelfMaybeWithMut(const clang::CXXMethodDecl *decl);

  std::string GetCtorName(clang::CXXConstructorDecl *decl);

  virtual void ConvertCXXRecordMethods(clang::CXXRecordDecl *decl);

  virtual void ConvertLateInstantiatedMethods(clang::CXXRecordDecl *decl);

  virtual std::string DestroyMembers(const clang::CXXRecordDecl *decl);

  bool NeedsScopedDestructor(const clang::VarDecl *decl) const;

  virtual void EmitScopedDestructor(const clang::VarDecl *decl);

  void EmitDeallocation(clang::CXXDeleteExpr *expr,
                        const std::string &argument_as_string);

  virtual void SetUFCSReceiver(clang::Expr *base, bool is_arrow,
                               const clang::CXXMethodDecl *method);

  void ConvertUserOperatorCall(clang::CXXOperatorCallExpr *expr);

  virtual std::string GetUFCSName(const clang::CXXMethodDecl *method) const;

  virtual bool ThisIsRustPtr() const { return false; }

  virtual void ConvertCXXConstructorBody(clang::CXXConstructorDecl *decl);
  // The object that decl constructs, before its body runs.
  void EmitConstructorInit(clang::CXXConstructorDecl *decl);

  virtual bool VisitCXXConstructorDecl(clang::CXXConstructorDecl *decl);

  virtual bool VisitFieldDecl(clang::FieldDecl *decl);

  virtual bool VisitNamespaceDecl(clang::NamespaceDecl *decl);

  bool VisitLinkageSpecDecl(clang::LinkageSpecDecl *decl);

  void ConvertDeclContext(clang::DeclContext *decl);

  virtual bool VisitTypedefDecl(clang::TypedefDecl *decl);
  virtual bool VisitTypeAliasDecl(clang::TypeAliasDecl *decl);
  virtual bool VisitTypeAliasTemplateDecl(clang::TypeAliasTemplateDecl *decl);

  bool VisitStaticAssertDecl(clang::StaticAssertDecl *decl);
  bool VisitConceptDecl(clang::ConceptDecl *decl);

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
        llvm::function_ref<std::pair<std::string, std::string>(clang::QualType)>
            materialize_fn);

    // str, preceded by the bindings of the temporaries that it refers to.
    std::string Wrap(std::string str) const;

  private:
    std::vector<std::string> materialized_refs_;
  };

  // args are the arguments of expr, including its receiver.
  std::string
  ConvertIRFragment(const std::vector<TranslationRule::BodyFragment> &fragments,
                    llvm::ArrayRef<clang::TemplateArgument> bindings,
                    clang::Expr *expr, llvm::ArrayRef<clang::Expr *> args,
                    TempMaterializationCtx *ctx);

  std::string
  ConvertGenericCall(const TranslationRule::GenericCallFragment &call,
                     llvm::ArrayRef<clang::TemplateArgument> bindings,
                     clang::Expr *expr, llvm::ArrayRef<clang::Expr *> args,
                     TempMaterializationCtx *ctx);

  std::string ConvertConversion(clang::QualType to, clang::QualType from,
                                bool is_move, const std::string &ptr);

  virtual const char *GetConversionTraitName() const;

  virtual std::string DerefPtrExpr(std::string_view ptr_expr,
                                   clang::QualType pointee_type);

  struct PlaceholderCtx {
    unsigned arg_idx;
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
      return access == TranslationRule::Access::kBorrowMut;
    }

    void dump() const;
  };

  // Converts the calls that are converted to something other than a call,
  // and returns whether expr is one.
  bool ConvertSpecialCall(clang::CallExpr *expr);

  // Converts the call expr, and returns the temporaries of the arguments of a
  // call translated by a rule.
  virtual std::optional<TempMaterializationCtx>
  ConvertCallExpr(clang::CallExpr *expr);

  // Emits the result of the call expr, which was converted to str.
  virtual void EmitCallResult(clang::CallExpr *expr, std::string str,
                              const TempMaterializationCtx *temps);

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
    clang::CallExpr *expr;
    bool is_variadic;
    bool is_fn_ptr_call;
    bool is_libc_passthrough;
  };

  CallInfo CollectCallInfo(clang::CallExpr *expr);

  void ConvertParamTy(clang::QualType param_type, clang::Expr *expr);

  virtual void ConvertParamTyConstCast(clang::QualType param_type,
                                       clang::Expr *expr);

  std::string ConvertScalarCast(std::string str, clang::QualType to);

  virtual std::string ConvertPointeeCast(std::string str,
                                         const clang::Expr *from,
                                         clang::QualType to);

  void ConvertFunctionPointerTransmute(clang::Expr *expr, clang::QualType type);

  void EmitHoistedArgs(CallInfo &info);

  void EmitArgList(const CallInfo &info);

  // The variadic arguments, each followed by a comma, converted to VaArg if
  // into_va_arg.
  void EmitVariadicArgs(llvm::ArrayRef<clang::Expr *> args, bool into_va_arg);

  void EmitCall(CallInfo &&info);

  void ConvertGenericCallExpr(clang::CallExpr *expr);

  virtual void EmitFnPtrCall(clang::CallExpr *expr);

  void ConvertLambdaCall(clang::CallExpr *expr,
                         const clang::CXXRecordDecl *lambda);

  virtual void
  ConvertLambdaToFunctionPointer(clang::Expr *lambda,
                                 const clang::CXXConversionDecl *conversion);

  void EmitLambdaFunctionPointer(const clang::CXXRecordDecl *lambda,
                                 clang::CXXMethodDecl *call_operator);

  virtual void
  ConvertFunctionToFunctionPointer(const clang::FunctionDecl *fn_decl);

  std::string GetFunctionRefName(const clang::FunctionDecl *fn_decl);

  std::string ConvertFnPtrCallee(clang::Expr *arg);
  virtual std::string ConvertFnPtrPlaceholder(clang::Expr *arg);

  // Option<fn> implements Copy
  virtual bool FunctionPointerImplementsCopy() const { return true; }

  bool TypeIsCopyable(clang::QualType ty) {
    if (ty->isFunctionPointerType() || ty->isFunctionType()) {
      return FunctionPointerImplementsCopy();
    }
    if (ty->isBuiltinType() || ty->isEnumeralType()) {
      return true;
    }
    if (ty->getAsRecordDecl()) {
      return RecordDerivesCopy(ty);
    }
    return false;
  }

  virtual void ConvertVAArgCall(clang::CallExpr *expr);

  virtual void ConvertVariadicArg(clang::Expr *arg);

  bool VisitCallExpr(clang::CallExpr *expr);

  virtual bool VisitIntegerLiteral(clang::IntegerLiteral *expr);

  virtual bool VisitFloatingLiteral(clang::FloatingLiteral *expr);

  virtual bool VisitCharacterLiteral(clang::CharacterLiteral *expr);

  std::string GetCodeUnitArrayLiteral(const clang::StringLiteral *expr);
  bool IsArrayInitContext() const;

  std::string GetEscapedUTF8CharLiteral(clang::Expr *expr) const;

  std::string GetEscapedStringLiteral(clang::Expr *expr,
                                      uint64_t pad_nulls = 0) const;
  virtual bool VisitStringLiteral(clang::StringLiteral *expr);

  bool VisitSourceLocExpr(clang::SourceLocExpr *expr);

  virtual std::string GetSourceFileAsString(clang::QualType type);

  virtual bool VisitCXXBoolLiteralExpr(clang::CXXBoolLiteralExpr *expr);

  void EmitBoolLiteral(bool value);

  // The value of expr, an integer constant expression.
  void EmitEvaluatedInt(clang::Expr *expr);

  void EmitNullPointer();

  bool VisitCXXNoexceptExpr(clang::CXXNoexceptExpr *expr);

  void ConvertIntegerToEnumeralCast(clang::Expr *to, clang::Expr *from);

  void ConvertIntegralToBooleanCast(clang::ImplicitCastExpr *expr);

  virtual bool VisitImplicitCastExpr(clang::ImplicitCastExpr *expr);

  virtual bool VisitExplicitCastExpr(clang::ExplicitCastExpr *expr);

  virtual bool VisitBinaryOperator(clang::BinaryOperator *expr);
  bool VisitCXXRewrittenBinaryOperator(clang::CXXRewrittenBinaryOperator *expr);

  virtual void ConvertBinaryOperator(clang::BinaryOperator *expr);

  // The value that the compound assignment expr assigns to its left operand,
  // whose type differs from the computation type.
  std::string ConvertCompoundAssignValue(clang::CompoundAssignOperator *expr);

  virtual bool ConvertIncAndDec(clang::UnaryOperator *expr);

  virtual bool VisitUnaryOperator(clang::UnaryOperator *expr);

  virtual bool VisitStmtExpr(clang::StmtExpr *expr);

  bool ConvertLValueConditional(clang::ConditionalOperator *expr);

  virtual bool VisitConditionalOperator(clang::ConditionalOperator *expr);

  // A branch of a conditional operator of the given type.
  virtual void ConvertConditionalBranch(clang::Expr *expr,
                                        clang::QualType type);

  virtual bool VisitDeclRefExpr(clang::DeclRefExpr *expr);

  virtual void ConvertDeclRefValue(clang::Expr *expr, clang::ValueDecl *decl);

  std::string ConvertDeclRef(clang::Expr *expr, clang::ValueDecl *decl);

  virtual bool VisitParenExpr(clang::ParenExpr *expr);

  void ConvertMemberExpr(clang::MemberExpr *expr);

  static const clang::Expr *GetFieldReadCall(const clang::MemberExpr *expr);

  virtual bool VisitMemberExpr(clang::MemberExpr *expr);

  virtual bool VisitCXXThisExpr(clang::CXXThisExpr *expr);

  virtual bool VisitInitListExpr(clang::InitListExpr *expr);
  virtual void ConvertScalarInitList(clang::InitListExpr *expr);
  // is_empty: whether the list, as written, has no initializers.
  virtual void ConvertRecordInitList(clang::InitListExpr *expr, bool is_empty);
  virtual void ConvertUnionInitList(clang::InitListExpr *expr,
                                    const clang::FieldDecl *field);
  virtual void ConvertArrayInitList(clang::InitListExpr *expr);
  bool VisitOpaqueValueExpr(clang::OpaqueValueExpr *expr);
  bool VisitArrayInitIndexExpr(clang::ArrayInitIndexExpr *expr);
  virtual bool VisitArrayInitLoopExpr(clang::ArrayInitLoopExpr *expr);

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

  virtual bool VisitSizeOfPackExpr(clang::SizeOfPackExpr *expr);

  virtual bool
  VisitConceptSpecializationExpr(clang::ConceptSpecializationExpr *expr);
  virtual bool VisitRequiresExpr(clang::RequiresExpr *expr);

  virtual bool VisitOffsetOfExpr(clang::OffsetOfExpr *expr);

  virtual bool VisitEnumDecl(clang::EnumDecl *decl);

  virtual std::string EnumeratorName(const clang::EnumConstantDecl *decl) const;

  virtual bool VisitCXXDefaultArgExpr(clang::CXXDefaultArgExpr *expr);
  virtual bool VisitConstantExpr(clang::ConstantExpr *expr);

  virtual bool VisitLambdaExpr(clang::LambdaExpr *expr);

  virtual void ConvertCapturelessLambda(const clang::CXXRecordDecl *decl);

  virtual const char *LambdaMacro() const { return "lambda_unsafe!"; }

  virtual void ConvertLambdaCapture(const clang::FieldDecl *field,
                                    clang::Expr *init);

  void ConvertLambdaClosure(const clang::CXXRecordDecl *decl,
                            clang::CXXMethodDecl *call_operator);

  virtual bool VisitImplicitValueInitExpr(clang::ImplicitValueInitExpr *expr);
  virtual bool VisitCXXScalarValueInitExpr(clang::CXXScalarValueInitExpr *expr);

  virtual bool VisitSwitchStmt(clang::SwitchStmt *stmt);

  void EmitSwitchArm(const SwitchArm &arm, bool is_default);

  bool ConvertSwitchCaseCondition(clang::SwitchCase *stmt);

  virtual bool VisitVAArgExpr(clang::VAArgExpr *expr);

  virtual bool VisitCXXDefaultInitExpr(clang::CXXDefaultInitExpr *expr);

  virtual bool VisitPredefinedExpr(clang::PredefinedExpr *expr);

  virtual bool VisitClassTemplateDecl(clang::ClassTemplateDecl *decl);

  bool TraverseClassTemplateSpecializationDecl(
      clang::ClassTemplateSpecializationDecl *) {
    return true;
  }

  bool TraverseExplicitInstantiationDecl(clang::ExplicitInstantiationDecl *) {
    return true;
  }

  virtual bool
  VisitCXXStdInitializerListExpr(clang::CXXStdInitializerListExpr *expr);

protected:
#define StrCat(...) _StrCat(__FUNCTION__, __LINE__, __VA_ARGS__)

  inline bool is_empty(char c) { return false; }
  inline bool is_empty(const char *s) { return s == nullptr || *s == '\0'; }
  template <size_t N> inline bool is_empty(const char (&s)[N]) {
    return s[0] == '\0';
  }
  template <typename T> inline bool is_empty(const T &s) { return s.empty(); }

  template <typename... Ts>
  inline void _StrCat(const char *func, int line, const Ts &...vals) {
    log() << '[' << func << ':' << line << "] ";
    ((log() << vals << '\n', *rs_code_ += vals,
      (is_empty(vals) ? void() : void(*rs_code_ += ' '))),
     ...);
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

  template <auto kOpen, auto kClose> class PushDelim {
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
  using PushLazyType = PushDelim<token::kLazyCellType, token::kGt>;
  using PushLazyInit = PushDelim<token::kLazyCellNew, token::kCloseParen>;

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

  virtual std::string ConvertPointeeType(clang::QualType ptr_type);

  virtual bool Convert(clang::Decl *decl);
  virtual bool Convert(clang::Stmt *stmt);
  virtual bool Convert(clang::Expr *expr,
                       std::optional<clang::QualType> implicit_convert_to = {});

  virtual std::string GetDefaultAsString(clang::QualType qual_type);

  virtual std::string GetArrayDefaultAsString(clang::QualType qual_type);

  virtual std::string GetDefaultAsStringFallback(clang::QualType qual_type);

  virtual std::string ConvertVarDefaultInit(const clang::VarDecl *decl);

  virtual std::string
  GetOverloadedFunctionName(const clang::CXXMethodDecl *decl);

  virtual std::string GetRecordName(const clang::NamedDecl *decl) const;

  virtual std::vector<const char *>
  GetStructAttributes(const clang::RecordDecl *decl);

  virtual std::string GetUnsafeTypeAsString(clang::QualType qual_type) const;

  virtual bool NeedsMut(const clang::VarDecl *decl, clang::QualType type,
                        llvm::StringRef name) const;

  virtual void ConvertVarInit(clang::QualType qual_type, clang::Expr *expr);

  // The initial value of a field of a struct: `init`, or the default value
  // if null.
  virtual void ConvertFieldInit(const clang::FieldDecl *field,
                                clang::Expr *init);

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

  void ConvertParamList(llvm::ArrayRef<clang::ParmVarDecl *> params,
                        bool is_variadic);

  virtual void ConvertFunctionReturnType(clang::FunctionDecl *decl);

  virtual void ConvertFunctionMain(const clang::FunctionDecl *decl,
                                   const std::string_view main_function_name);

  virtual void ConvertAbstractClass(clang::CXXRecordDecl *decl);

  void ConvertCXXMethodDecls(const clang::CXXRecordDecl *decl,
                             const std::string_view signature,
                             bool (*predicate)(clang::CXXMethodDecl *));

  void ConvertVirtualMethods(clang::CXXRecordDecl *decl);

  bool ConvertOutOfLineVirtualMethod(clang::CXXMethodDecl *decl);

  void AddOrdTrait(const clang::CXXRecordDecl *decl);

  virtual void ConvertOrdAndPartialOrdTraits(const clang::CXXRecordDecl *decl,
                                             const clang::FunctionDecl *eq,
                                             const clang::FunctionDecl *lt,
                                             const clang::FunctionDecl *cmp);

  std::string GetEqBody(const clang::CXXRecordDecl *decl,
                        const clang::FunctionDecl *eq,
                        const clang::FunctionDecl *lt,
                        const clang::FunctionDecl *cmp, std::string_view lhs,
                        std::string_view rhs);

  void ConvertOrdAndPartialOrdTraitsBase(std::string_view cmp_body,
                                         std::string_view eq_body,
                                         std::string_view record_name);

  std::string GetComparisonCall(const clang::FunctionDecl *op,
                                const clang::CXXRecordDecl *decl,
                                std::string_view lhs, std::string_view rhs);

  std::string GetComparisonBool(const clang::FunctionDecl *op,
                                const clang::CXXRecordDecl *decl,
                                std::string_view lhs, std::string_view rhs);

  // The argument for param of a comparison operator of decl, from an operand
  // of the comparison.
  virtual std::string GetComparisonArg(const clang::ParmVarDecl *param,
                                       const clang::CXXRecordDecl *decl,
                                       std::string_view value);

  virtual std::string
  GetComparisonResultReceiver(const clang::CXXMethodDecl *method,
                              bool returns_ref, std::string_view result);

  virtual std::string GetComparisonReceiver(const clang::CXXMethodDecl *method,
                                            std::string_view lhs);

  virtual void AddCloneTrait(const clang::RecordDecl *decl);

  virtual bool RecordImplementsClone(const clang::RecordDecl *decl);

  virtual void AddDefaultTrait(const clang::RecordDecl *decl);

  virtual void AddDefaultTraitForUnion(const clang::RecordDecl *decl);

  void EmitDefaultStructLiteral(const clang::RecordDecl *decl);

  // Emits `impl Default for decl`, whose default() returns what body emits.
  void EmitDefaultImpl(const clang::RecordDecl *decl,
                       llvm::function_ref<void()> body);

  virtual void EmitByteSizeAttr(const clang::RecordDecl *decl);

  virtual void
  ConvertUnsignedArithBinaryOperator(clang::BinaryOperator *binary_operator,
                                     clang::Expr *expr);

  virtual void ConvertAddrOf(clang::Expr *expr, clang::QualType pointer_type);

  virtual void ConvertDeref(clang::Expr *expr);

  void EmitDeref(std::string inner, clang::QualType pointee_type);

  virtual void ConvertArrow(clang::Expr *expr);

  virtual void ConvertCast(clang::QualType qual_type,
                           int line = __builtin_LINE());

  virtual bool ConvertCXXOperatorCallExpr(clang::CXXOperatorCallExpr *expr);

  std::string GetMappedAsString(clang::Expr *expr, clang::Expr **args = nullptr,
                                unsigned num_args = 0,
                                TempMaterializationCtx *ctx = nullptr);

  std::string ConvertPlaceholder(clang::Expr *expr, clang::Expr *arg,
                                 const PlaceholderCtx &ph_ctx);

  // Moves the value out of `arg`, leaving a default value behind.
  virtual std::string ConvertTake(clang::Expr *arg);

  // The place of `arg`, which is borrowed mutably by a rule.
  virtual std::string ConvertBorrowMut(clang::Expr *arg);

  // The places that the arguments of the rule being converted are borrowed
  // through, with the names they are borrowed as, e.g., a pointer and the
  // closure parameter of its with_mut.
  std::vector<std::pair<std::string, std::string>> mut_borrows_;

  // The variables that arguments of rules are bound to by value, whose
  // address is not taken.
  std::unordered_set<const clang::VarDecl *> bound_values_;

  std::string ConvertVariadicTail(clang::Expr *expr,
                                  llvm::ArrayRef<clang::Expr *> all_args);

  std::string ConvertInitFragment(clang::Expr *expr,
                                  llvm::ArrayRef<clang::Expr *> all_args);

  void ConvertConstructFromArgs(clang::QualType type,
                                llvm::ArrayRef<clang::Expr *> args,
                                clang::SourceLocation loc);

  void ConvertInit(clang::QualType type, clang::Expr *init);

  virtual void ConvertConstructedValue(clang::QualType type,
                                       clang::CXXConstructExpr *ctor);

  virtual std::string ConvertMappedMethodCall(
      clang::Expr *expr, const TranslationRule::MethodCallFragment &mc,
      llvm::ArrayRef<clang::TemplateArgument> bindings,
      llvm::ArrayRef<clang::Expr *> args, TempMaterializationCtx *ctx);

  virtual std::string AccessLValueObject(clang::MemberExpr *member);

  virtual void ConvertGenericBinaryOperator(clang::BinaryOperator *expr);

  virtual bool IsReferenceType(const clang::Expr *expr) const;

  bool ExprMapsValueToPointer(const clang::CallExpr *expr) const;

  virtual bool RecordDerivesDefault(const clang::RecordDecl *decl);

  virtual bool TypeDerivesDefault(clang::QualType qual_type);

  bool RecordDerivesCopy(clang::QualType type);

  bool IsPassThroughRule(clang::Expr *expr);

  // Whether the rule of expr takes its argument number index, arg, a container
  // in C++, as a pointer to its elements rather than to the container.
  virtual bool ParamIsElementPtr(clang::Expr * /*expr*/, clang::Expr * /*arg*/,
                                 unsigned /*index*/) {
    return false;
  }

  bool RecordHasCopyableFields(const clang::RecordDecl *decl);

  bool ShouldReplaceWithMappedBody(const clang::ValueDecl *decl) const;

  std::string *rs_code_;
  clang::ASTContext &ctx_;
  std::unique_ptr<clang::MangleContext> mangle_context_;
  clang::FunctionDecl *curr_function_ = nullptr;
  bool InConstructor() const {
    return clang::isa_and_present<clang::CXXConstructorDecl>(curr_function_);
  }
  bool in_function_formals_ = false;
  enum class MethodTarget : uint8_t {
    ValueImpl,
    TraitDecl,
    TraitDefault,
    PtrImpl,
  };
  MethodTarget method_target_ = MethodTarget::ValueImpl;

  std::string ufcs_receiver_;
  bool in_const_initializer_ = false;
  std::optional<bool> autoref_mut_;

  std::vector<clang::Expr *> curr_for_inc_;
  // The names of the iterators of range-based for loops.
  std::unordered_map<const clang::ValueDecl *, std::string>
      range_iterator_names_;
  std::vector<clang::QualType> curr_init_type_;

  enum class BreakTarget : int8_t { Loop, FallthroughSwitch, Switch };
  std::vector<BreakTarget> break_target_;

  bool isSwitchBreak() const {
    return !break_target_.empty() &&
           break_target_.back() == BreakTarget::Switch;
  }

  // Local variables hoisted outside a goto_block so that all labels can see and
  // use the variables.
  std::unordered_set<const clang::VarDecl *> hoisted_decls_;

  // A call whose result a field is read from, e.g., `f(g()).x`, and the
  // bindings of the temporaries of its arguments, which the read is in the
  // block of, so that they are alive during it.
  struct FieldReadCall {
    const clang::Expr *call;
    std::string temporary_bindings;
  };
  FieldReadCall *field_read_call_ = nullptr;
  // Whether the bindings of temps, of the arguments of call, are deferred to
  // the read of a field of its result.
  bool DeferTemporaries(const clang::Expr *call,
                        const TempMaterializationCtx *temps);

  // The number of array init loops being converted, which copy arrays
  // element by element, e.g., in implicit copy constructors, and the number
  // of loops outside each of them, whose indices their common expressions
  // use.
  unsigned array_init_depth_ = 0;
  std::unordered_map<const clang::OpaqueValueExpr *, unsigned>
      array_init_outer_depth_;
  std::string ArrayInitIndexName() const;

  unsigned materialized_temp_id_ = 0;
  std::string NewTempName() {
    return std::format("__tmp_{}", materialized_temp_id_++);
  }
  std::string *materialized_temp_bindings_ = nullptr;
  class HoistMaterializedTempBindings {
    Converter &c;
    std::string *prev;
    std::string bindings;
    std::optional<Buffer> buf;
    bool as_block;

  public:
    explicit HoistMaterializedTempBindings(Converter &c, bool as_block = false)
        : c(c), prev(c.materialized_temp_bindings_), buf(c),
          as_block(as_block) {
      c.materialized_temp_bindings_ = &bindings;
    }
    ~HoistMaterializedTempBindings() {
      c.materialized_temp_bindings_ = prev;
      std::string body = std::move(*buf).str();
      buf.reset();

      if (as_block && !bindings.empty()) {
        c.StrCat('{', bindings, body, '}');
        return;
      }
      c.StrCat(bindings, body);
    }
    HoistMaterializedTempBindings(const HoistMaterializedTempBindings &) =
        delete;
    HoistMaterializedTempBindings &
    operator=(const HoistMaterializedTempBindings &) = delete;
  };

  static std::unordered_set<std::string> decl_ids_;

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
  struct DeferredBlock {
    std::string header;
    std::string body;
  };
  static std::map<std::string, DeferredBlock> virtual_methods_;

  static void EmitDeferredBlock(const DeferredBlock &block, std::string &out);

  DeferredBlock &VirtualMethodsFor(const clang::CXXRecordDecl *decl);

  std::string hoisted_records_;

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
    bool pushed;
    PushExprKind(Converter &c, ExprKind k, bool enabled = true,
                 const char *file = __builtin_FILE(),
                 int line = __builtin_LINE())
        : c(c), pushed(enabled) {
      if (pushed) {
        c.curr_expr_kind_.push_back(k);
      }
      log() << "PushExprKind " << file << ':' << line << ' ';
      c.dump_expr_kinds();
      log() << '[';
      for (const auto k : c.curr_expr_kind_) {
        log() << c.expr_kind_to_string(k) << ", ";
      }
      log() << "]\n";
    }
    ~PushExprKind() {
      if (pushed) {
        c.curr_expr_kind_.pop_back();
      }
    }
  };

  // Whether the converted expression evaluates to a new value, which can be
  // moved, rather than to an existing one, which must be copied.
  enum class ComputedExprType : uint8_t {
    Fresh,
    NotFresh,
    Unknown,
    Pending,
  };
  ComputedExprType computed_expr_type_ = ComputedExprType::Unknown;

  bool isFresh() const {
    return computed_expr_type_ == ComputedExprType::Fresh;
  }

  void MarkFresh() { computed_expr_type_ = ComputedExprType::Fresh; }
  void MarkNotFresh() { computed_expr_type_ = ComputedExprType::NotFresh; }
  // Emits str, which evaluates to a new value.
  void EmitFresh(std::string_view str);
  // str, cloned unless the expression it was converted from is fresh.
  std::string MakeFresh(std::string str);
  // A read of a value of the given type is fresh if the value is copied.
  void SetValueFreshness(clang::QualType type);

  std::string ConvertLValue(clang::Expr *expr);
  std::string
  ConvertRValue(clang::Expr *expr,
                std::optional<clang::QualType> implicit_convert_to = {},
                int line = __builtin_LINE());
  virtual std::string
  ConvertFreshRValue(clang::Expr *expr,
                     std::optional<clang::QualType> implicit_convert_to = {});
  virtual std::string
  ConvertFreshPointer(clang::Expr *expr,
                      std::optional<clang::QualType> implicit_convert_to = {});
  virtual std::string ConvertFreshObject(clang::Expr *expr);

  // A pointer of param_type to arg, which is converted to implicit_convert_to
  // if it is a temporary.
  virtual std::string
  ConvertPointerReceiver(clang::Expr *arg, std::string_view param_type,
                         std::optional<clang::QualType> implicit_convert_to);

  // A pointer to the elements of the container arg.
  virtual std::string ConvertElementPtr(clang::Expr *arg) {
    llvm_unreachable("no pointers to elements in this model");
  }

  // A pointer to a new temporary with the value of the prvalue expr, which
  // lives until the end of the statement.
  virtual std::string
  ConvertTemporaryPointer(clang::Expr *expr,
                          std::optional<clang::QualType> implicit_convert_to);

  std::string
  ConvertPointer(clang::Expr *expr,
                 std::optional<clang::QualType> implicit_convert_to = {},
                 int line = __builtin_LINE());

  /// Materialize a temporary for a prvalue bound to a reference parameter.
  /// Returns (binding_code, ref_expression).
  virtual std::pair<std::string, std::string>
  MaterializeTemp(const std::string &binding_name, clang::QualType param_type,
                  clang::Expr *expr);

  /// Emits binding_code to materialized_temp_bindings_.
  /// Returns ref_expression.
  std::string EmitMaterializedTempBinding(clang::QualType param_type,
                                          clang::Expr *expr);

  virtual const char *GetPointerDerefPrefix(clang::QualType pointee_type);

  TempMaterializationCtx CollectRefBindingTempArgs(clang::CallExpr *expr);

  bool IsCastRedundantInRust(clang::Expr *expr, clang::QualType target_type);

private:
  std::string getIntegerLiteral(clang::IntegerLiteral *expr, bool incl_type,
                                const clang::QualType *type = nullptr);
  const char *keyword_unsafe_;
  const char *keyword_mut_;
  std::vector<ExprKind> curr_expr_kind_;
  static std::unordered_map<std::string, std::string> inner_structs_;
  static std::unordered_set<std::string> globals_;
  static std::vector<std::string> global_inits_;
  clang::Sema *sema_ = nullptr;
};
} // namespace cpp2rust
