// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Attr.h>
#include <clang/AST/Decl.h>
#include <clang/AST/Expr.h>
#include <clang/AST/PrettyPrinter.h>
#include <clang/AST/RecursiveASTVisitor.h>
#include <clang/ASTMatchers/ASTMatchFinder.h>
#include <clang/ASTMatchers/ASTMatchers.h>
#include <clang/Frontend/FrontendActions.h>
#include <clang/Sema/Initialization.h>
#include <clang/Sema/Lookup.h>
#include <clang/Sema/Overload.h>
#include <clang/Sema/Sema.h>
#include <clang/Sema/Template.h>
#include <clang/Tooling/CompilationDatabase.h>
#include <clang/Tooling/Tooling.h>
#include <llvm/ADT/ArrayRef.h>
#include <llvm/ADT/SmallVector.h>
#include <llvm/ADT/StringRef.h>
#include <llvm/Support/CommandLine.h>
#include <llvm/Support/FormatVariadic.h>
#include <llvm/Support/JSON.h>
#include <llvm/Support/raw_ostream.h>

#include <cstdlib>
#include <filesystem>
#include <string>

#include "compat/platform_flags.h"
#include "converter/converter_lib.h"
#include "converter/mapper.h"

namespace fs = std::filesystem;

namespace cpp2rust {

class Callback : public clang::ast_matchers::MatchFinder::MatchCallback {
public:
  explicit Callback(llvm::json::Object &out) : out_(out) {}

  void init(clang::Sema &sema) {
    sema_ = &sema;
    clang::SourceManager &sm = sema.Context.getSourceManager();
    loc_ = sm.getLocForStartOfFile(sm.getMainFileID());
  }

  void run(const clang::ast_matchers::MatchFinder::MatchResult &R) override {
    assert(sema_);
    Mapper::PushASTContext scoped(*R.Context);
    clang::NamespaceDecl *ns = createNamespaceDecl();
    const clang::Sema::ContextRAII savedContext(*sema_, ns);

    if (auto func = R.Nodes.getNodeAs<clang::FunctionDecl>("func")) {
      const char *err = nullptr;
      if (auto body =
              clang::dyn_cast_or_null<clang::CompoundStmt>(func->getBody())) {
        if (body->size() != 1) {
          err = "body must contain exactly one statement (a return)";
        } else if (!clang::isa<clang::ReturnStmt>(*body->body_begin())) {
          err = "body must be a return statement";
        }
      } else {
        err = "body cannot be empty";
      }

      if (err) {
        llvm::errs() << "ERROR: " << func->getQualifiedNameAsString() << ": "
                     << err << '\n';
        std::exit(EXIT_FAILURE);
      }

      auto *rule = func;
      if (auto *tdecl = func->getDescribedFunctionTemplate()) {
        rule = instantiateFunctionRule(tdecl);
        assert(rule && "Instantiation failed");
      }

      auto *body = llvm::cast<clang::CompoundStmt>(rule->getBody());
      auto *ret = llvm::cast<clang::ReturnStmt>(*body->body_begin());
      auto src = Mapper::ToString(
          ret->getRetValue()->IgnoreUnlessSpelledInSource(), sema_->CurContext);
      assert(src != "Unhandled case in ToString");
      out_.try_emplace(rule->getQualifiedNameAsString(), std::move(src));
      return;
    }

    if (auto var = R.Nodes.getNodeAs<clang::TypedefNameDecl>("tvar")) {
      clang::QualType type = var->getUnderlyingType();
      if (auto *alias = llvm::dyn_cast<clang::TypeAliasDecl>(var)) {
        if (auto *tdecl = alias->getDescribedAliasTemplate()) {
          type = instantiateTypeRule(tdecl);
        }
      }
      auto src = Mapper::ToString(type, Mapper::ScalarSugar::kPreserve,
                                  sema_->CurContext);
      assert(src != "Unhandled case in ToString");
      out_.try_emplace(var->getQualifiedNameAsString(), std::move(src));
      return;
    }
  }

private:
  llvm::json::Object &out_;
  clang::Sema *sema_ = nullptr;
  clang::SourceLocation loc_;

  clang::NamespaceDecl *createNamespaceDecl() {
    auto &ctx = sema_->getASTContext();
    auto *tu = ctx.getTranslationUnitDecl();
    auto *ns = clang::NamespaceDecl::Create(ctx, tu, false, loc_, loc_, nullptr,
                                            nullptr, false);
    tu->addDecl(ns);
    return ns;
  }

  clang::RecordDecl *createRecordDecl(llvm::StringRef name) {
    bool owned = true;
    bool dependent = false;
    clang::CXXScopeSpec scope;
    clang::MultiTemplateParamsArg args;
    auto decl = sema_->ActOnTag(
        sema_->getCurScope(), clang::DeclSpec::TST_struct,
        clang::TagUseKind::Definition, loc_, scope,
        &sema_->Context.Idents.get(name), loc_, clang::ParsedAttributesView(),
        clang::AS_none, loc_, args, owned, dependent, loc_, false,
        clang::TypeResult(), false, false, clang::OffsetOfKind::Outside);
    assert(decl.isUsable() && "Record decl creation failed");
    auto *rdecl = decl.getAs<clang::RecordDecl>();

    rdecl->startDefinition();
    rdecl->completeDefinition();
    return rdecl;
  }

  static clang::QualType getNTTPType(const clang::TemplateArgument &arg) {
    switch (arg.getKind()) {
    case clang::TemplateArgument::Integral:
      return arg.getIntegralType();
    case clang::TemplateArgument::Declaration:
      return arg.getParamTypeForDecl();
    case clang::TemplateArgument::NullPtr:
      return arg.getNullPtrType();
    case clang::TemplateArgument::StructuralValue:
      return arg.getStructuralValueType();
    default:
      return clang::QualType();
    }
  }

  clang::QualType
  getTemplateIdType(clang::TemplateDecl *decl,
                    llvm::ArrayRef<clang::TemplateArgument> args) {
    clang::TemplateArgumentListInfo info(loc_, loc_);
    for (const auto &arg : args) {
      assert(!arg.getIsDefaulted());
      info.addArgument(
          sema_->getTrivialTemplateArgumentLoc(arg, getNTTPType(arg), loc_));
    }
    return sema_->CheckTemplateIdType(clang::ElaboratedTypeKeyword::None,
                                      clang::TemplateName(decl), loc_, info,
                                      sema_->getCurScope(),
                                      /*ForNestedNameSpecifier=*/false);
  }

  clang::QualType createAliasType(llvm::StringRef name, clang::QualType hint) {
    clang::ASTContext &ctx = sema_->Context;

    // required to print Tn instead of Tn<arg1, arg2, ...>
    auto *alias = clang::TypeAliasDecl::Create(
        ctx, sema_->CurContext, loc_, loc_, &ctx.Idents.get(name),
        ctx.getTrivialTypeSourceInfo(hint, loc_));
    sema_->CurContext->addDecl(alias);

    clang::QualType alias_t = ctx.getTypedefType(
        clang::ElaboratedTypeKeyword::None, std::nullopt, alias);

    if (auto *hdecl = hint->getAsCXXRecordDecl()) {
      hdecl->dropAttr<clang::PreferredNameAttr>();
      hdecl->addAttr(clang::PreferredNameAttr::CreateImplicit(
          ctx, ctx.getTrivialTypeSourceInfo(alias_t, loc_)));
    }
    return hint;
  }

  clang::QualType getSubstType(const clang::Sema::InstantiatingTemplate &Inst,
                               clang::QualType type,
                               llvm::ArrayRef<clang::TemplateArgument> args) {
    assert(!Inst.isInvalid() && "Invalid instantiation context");
    clang::MultiLevelTemplateArgumentList mtal;
    mtal.setKind(clang::TemplateSubstitutionKind::Rewrite);
    mtal.addOuterTemplateArguments(args);

    clang::TypeSourceInfo *tsi =
        sema_->SubstType(sema_->Context.getTrivialTypeSourceInfo(type), mtal,
                         loc_, clang::DeclarationName());
    assert(tsi && "Template argument type instantiation failed");
    return tsi->getType();
  }

  clang::QualType
  getDefaultArg(clang::TemplateDecl *decl,
                const clang::TemplateTypeParmDecl *parm,
                llvm::ArrayRef<clang::TemplateArgument> currentArgs) {
    clang::QualType type = parm->getDefaultArgument().getArgument().getAsType();
    if (!type->isDependentType()) {
      return type;
    }

    const clang::Sema::InstantiatingTemplate Inst(*sema_, loc_, decl);
    return getSubstType(Inst, type, currentArgs);
  }

  clang::DeclRefExpr *createConstexprDeclRefExpr(clang::QualType type,
                                                 clang::Expr *init,
                                                 llvm::StringRef name) {
    clang::ASTContext &ctx = sema_->Context;
    clang::VarDecl *decl = clang::VarDecl::Create(
        ctx, sema_->CurContext, loc_, loc_, &ctx.Idents.get(name),
        type.getNonReferenceType(), nullptr, clang::SC_Static);
    sema_->CurContext->addDecl(decl);
    decl->markUsed(ctx);
    decl->setConstexpr(true);

    if (!init) {
      init = new (ctx) clang::ImplicitValueInitExpr(type);
    }
    decl->setInit(init);

    const clang::DeclarationNameInfo nameInfo(decl->getDeclName(), loc_);
    return sema_->BuildDeclRefExpr(decl, decl->getType(), clang::VK_LValue,
                                   nameInfo, decl->getQualifierLoc());
  }

  clang::OpaqueValueExpr *createOpaqueValueExpr(clang::QualType type) {
    return new (sema_->Context) clang::OpaqueValueExpr(
        loc_, type.getNonReferenceType(),
        type->isRValueReferenceType() ? clang::VK_XValue : clang::VK_LValue);
  }

  void
  createTemplateArguments(clang::TemplateDecl *decl,
                          llvm::SmallVectorImpl<clang::TemplateArgument> &out) {
    for (clang::NamedDecl *param : *decl->getTemplateParameters()) {
      if (const auto *ttp =
              llvm::dyn_cast<clang::TemplateTypeParmDecl>(param)) {
        clang::QualType type;
        if (param->isTemplateParameterPack()) {
          out.emplace_back(clang::TemplateArgument::getEmptyPack());
          continue;
        }
        if (ttp->hasDefaultArgument()) {
          clang::QualType hint = getDefaultArg(decl, ttp, out);
          assert(!hint.isNull() && "Failed retrieving type hint");
          type = createAliasType(param->getName(), hint);
        } else {
          clang::RecordDecl *rdecl = createRecordDecl(param->getName());
          type = sema_->Context.getTagType(clang::ElaboratedTypeKeyword::None,
                                           rdecl->getQualifier(), rdecl, false);
        }
        assert(!type.isNull() && "Template type argument creation failed");
        out.emplace_back(type);
      } else if (const auto *nttp =
                     llvm::dyn_cast<clang::NonTypeTemplateParmDecl>(param)) {
        clang::QualType type = nttp->getType();
        if (type->isDependentType()) {
          const clang::Sema::InstantiatingTemplate Inst(*sema_, loc_, decl);
          type = getSubstType(Inst, type, out);
        }
        clang::Expr *hint = nullptr;
        if (nttp->hasDefaultArgument()) {
          auto *arg = nttp->getDefaultArgument().getArgument().getAsExpr();
          if (auto *init = llvm::dyn_cast<clang::DeclRefExpr>(arg)) {
            hint = llvm::cast<clang::VarDecl>(init->getDecl())->getInit();
            type = hint->getType();
          }
        }
        clang::DeclRefExpr *var =
            createConstexprDeclRefExpr(type, hint, param->getName());
        out.emplace_back(var, true);
      } else {
        assert(0 && "Unsupported template param kind");
      }
    }
  }

  clang::FunctionDecl *
  instantiateFunctionRule(clang::FunctionTemplateDecl *decl) {
    llvm::SmallVector<clang::TemplateArgument, 8> args;
    createTemplateArguments(decl, args);
    auto *inst = sema_->InstantiateFunctionDeclaration(
        decl, clang::TemplateArgumentList::CreateCopy(sema_->Context, args),
        loc_);
    assert(inst && "Function rule instantiation failed");
    sema_->InstantiateFunctionDefinition(loc_, inst);
    return inst;
  }

  clang::QualType instantiateTypeRule(clang::TypeAliasTemplateDecl *decl) {
    llvm::SmallVector<clang::TemplateArgument, 8> args;
    createTemplateArguments(decl, args);

    clang::QualType type = getTemplateIdType(decl, args);
    assert(!type.isNull() && "Type rule instantiation failed");

    const auto *tst = type->getAs<clang::TemplateSpecializationType>();
    assert(tst && tst->isTypeAlias());
    return tst->getAliasedType();
  }
};

class ActionFactory : public clang::tooling::FrontendActionFactory {
public:
  explicit ActionFactory(llvm::json::Object &out) : cb_(out) {
    using namespace clang::ast_matchers;
    finder_.addMatcher(
        typedefNameDecl(matchesName("(^|::)t[0-9]+$"), isExpansionInMainFile())
            .bind("tvar"),
        &cb_);

    finder_.addMatcher(
        functionDecl(isDefinition(), matchesName("(^|::)f[0-9]+$"),
                     isExpansionInMainFile(), unless(isTemplateInstantiation()))
            .bind("func"),
        &cb_);
  }

  std::unique_ptr<clang::FrontendAction> create() override {
    class ASTConsumer : public clang::ASTConsumer {
    public:
      explicit ASTConsumer(std::unique_ptr<clang::ASTConsumer> AC,
                           clang::CompilerInstance &CI, Callback *CB)
          : AC_(std::move(AC)), CI_(&CI), CB_(CB) {}

      void HandleTranslationUnit(clang::ASTContext &ctx) override {
        auto &DE = CI_->getDiagnostics();
        if (DE.hasErrorOccurred()) {
          std::exit(EXIT_FAILURE);
        }
        CB_->init(CI_->getSema());
        AC_->HandleTranslationUnit(ctx);
        if (DE.hasErrorOccurred()) {
          std::exit(EXIT_FAILURE);
        }
      }

    private:
      std::unique_ptr<clang::ASTConsumer> AC_;
      clang::CompilerInstance *CI_;
      Callback *CB_;
    };

    class Wrapped : public clang::ASTFrontendAction {
      clang::ast_matchers::MatchFinder &F_;
      Callback *CB_;

    public:
      explicit Wrapped(clang::ast_matchers::MatchFinder &MF, Callback &CB)
          : F_(MF), CB_(&CB) {}

      std::unique_ptr<clang::ASTConsumer>
      CreateASTConsumer(clang::CompilerInstance &CI, llvm::StringRef) override {
        return std::make_unique<ASTConsumer>(F_.newASTConsumer(), CI, CB_);
      }
    };
    return std::make_unique<Wrapped>(finder_, cb_);
  }

private:
  clang::ast_matchers::MatchFinder finder_;
  Callback cb_;
};

void Extract(const std::filesystem::path &src_path, llvm::json::Object &out,
             llvm::ArrayRef<llvm::StringRef> cxx_flags) {
  auto flags = getPlatformClangBeginFlags();
  flags.insert(flags.end(), cxx_flags.begin(), cxx_flags.end());
  auto end_flags = getPlatformClangEndFlags();
  flags.insert(flags.end(), end_flags.begin(), end_flags.end());
  clang::tooling::FixedCompilationDatabase compilations(".", flags);
  ActionFactory factory(out);
  clang::tooling::ClangTool tool(compilations, {src_path.string()});
  tool.run(&factory);
}

} // namespace cpp2rust

namespace {

llvm::cl::OptionCategory cat("cpp-rule-preprocessor options");

llvm::cl::opt<std::string>
    SrcDir("dir",
           llvm::cl::desc("Path to a rule directory containing src.c and/or "
                          "src.cpp."),
           llvm::cl::value_desc("rule-dir"), llvm::cl::Required,
           llvm::cl::cat(cat));

llvm::cl::opt<std::string>
    OutPath("out", llvm::cl::desc("Path of the ir_src.json file to write."),
            llvm::cl::value_desc("out.json"), llvm::cl::Required,
            llvm::cl::cat(cat));

llvm::cl::list<std::string> CXXFlags("cxxflags",
                                     llvm::cl::desc("Additional CXXFLAGS"),
                                     llvm::cl::value_desc("cxxflags"),
                                     llvm::cl::ZeroOrMore, llvm::cl::cat(cat));

} // namespace

int main(int argc, char *argv[]) {
  llvm::cl::HideUnrelatedOptions(cat);
  llvm::cl::ParseCommandLineOptions(argc, argv);

  llvm::SmallVector<llvm::StringRef, 4> cxx_flags = {
      "-Wno-everything",
      "-I",
      RULES_LIB_INCLUDE_DIR,
  };
  cxx_flags.append(CXXFlags.begin(), CXXFlags.end());

  fs::path dir = SrcDir.getValue();
  llvm::json::Object root;
  for (const char *name : {"src.c", "src.cpp"}) {
    auto path = dir / name;
    if (!fs::exists(path)) {
      continue;
    }
    llvm::errs() << "Preprocessing " << path.string() << '\n';
    llvm::json::Object file_root;
    cpp2rust::Extract(path, file_root, cxx_flags);
    for (auto &[k, v] : file_root) {
      if (!root.try_emplace(k, std::move(v)).second) {
        llvm::errs() << "ERROR: rule name " << k.str()
                     << " defined in multiple files in " << dir.string()
                     << '\n';
        return EXIT_FAILURE;
      }
    }
  }

  fs::path out_path = OutPath.getValue();
  std::error_code ec;
  llvm::raw_fd_ostream out(out_path.string(), ec);
  if (ec) {
    llvm::errs() << "ERROR: failed to open " << out_path.string() << ": "
                 << ec.message() << '\n';
    return EXIT_FAILURE;
  }
  out << llvm::formatv("{0:2}", llvm::json::Value(std::move(root))) << '\n';
  return EXIT_SUCCESS;
}
