// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
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
#include <llvm/Support/CommandLine.h>
#include <llvm/Support/FormatVariadic.h>
#include <llvm/Support/JSON.h>
#include <llvm/Support/raw_ostream.h>

#include <cstdlib>
#include <filesystem>
#include <string>

#include "compat/platform_flags.h"
#include "converter/mapper.h"

namespace fs = std::filesystem;

namespace cpp2rust {

enum LookupKind { RegularName, CXXMethodName, CXXConstructorName, ADL };

struct LookupInfo {
  clang::DeclarationName name;
  LookupKind kind;
  llvm::ArrayRef<clang::TemplateArgumentLoc> explicitArgs;

  LookupInfo(const clang::Expr *expr) {
    if (const auto *ul = llvm::dyn_cast<clang::UnresolvedLookupExpr>(expr)) {
      clang::DeclarationName dname = ul->getName();
      name = dname;
      if (ul->requiresADL()) {
        kind = LookupKind::ADL;
      } else {
        kind = LookupKind::RegularName;
      }
      explicitArgs = ul->template_arguments();
    } else if (const auto *dm =
                   llvm::dyn_cast<clang::CXXDependentScopeMemberExpr>(expr)) {
      name = dm->getMember();
      kind = LookupKind::CXXMethodName;
      explicitArgs = dm->template_arguments();
    } else if (const auto *dref =
                   llvm::dyn_cast<clang::DependentScopeDeclRefExpr>(expr)) {
      clang::DeclarationName dname = dref->getDeclName();
      if (name.getNameKind() ==
          clang::DeclarationName::NameKind::CXXConstructorName) {
        name = dname;
        kind = LookupKind::CXXConstructorName;
      } else {
        assert(0 && "Unsupported dref name kind");
      }
    } else if (llvm::isa<clang::CXXUnresolvedConstructExpr>(expr)) {
      kind = LookupKind::CXXConstructorName;
    } else {
      expr->dump();
      assert(0 && "Unsupported lookup expression");
    }
  }
};

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
    if (auto var = R.Nodes.getNodeAs<clang::TypeAliasDecl>("tvar")) {
      clang::QualType type;
      if (auto *tdecl = var->getDescribedAliasTemplate()) {
        type = lookupType(tdecl);
      } else {
        type = var->getUnderlyingType();
      }
      out_.try_emplace(var->getQualifiedNameAsString(), Mapper::ToString(type));
      return;
    }

    if (auto func = R.Nodes.getNodeAs<clang::FunctionDecl>("func")) {
      auto add = [&](std::string &&src) {
        out_.try_emplace(func->getQualifiedNameAsString(), std::move(src));
      };

      if (const auto *fcall = R.Nodes.getNodeAs<clang::CallExpr>("fcall")) {
        if (fcall->getDirectCallee()) {
          add(Mapper::ToString(fcall));
          return;
        }

        LookupInfo lookup(fcall->getCallee());
        clang::NamedDecl *decl =
            lookupCalledDecl(func->getDescribedFunctionTemplate(), lookup);
        add(Mapper::ToString(decl));
        return;
      }
      if (const auto *ctor =
              R.Nodes.getNodeAs<clang::CXXConstructExpr>("ctor")) {
        if (ctor->getConstructor()) {
          add(Mapper::ToString(ctor));
          return;
        }
      }
      if (const auto *muse = R.Nodes.getNodeAs<clang::MemberExpr>("muse")) {
        if (llvm::isa<clang::FieldDecl>(muse->getMemberDecl())) {
          add(Mapper::ToString(muse));
          return;
        }
      }
      if (const auto *um =
              R.Nodes.getNodeAs<clang::UnresolvedMemberExpr>("umuse")) {
        add(Mapper::ToString(um));
        return;
      }
      if (R.Nodes.getNodeAs<clang::DeclRefExpr>("declref")) {
        if (const auto *enum_val =
                R.Nodes.getNodeAs<clang::EnumConstantDecl>("enum_val")) {
          add(Mapper::ToString(enum_val));
          return;
        } else if (const auto *decl =
                       R.Nodes.getNodeAs<clang::VarDecl>("decl")) {
          add(Mapper::ToString(decl));
          return;
        }
      }
      if (const auto *uop =
              R.Nodes.getNodeAs<clang::UnaryOperator>("udeclref")) {
        add(Mapper::ToString(uop));
        return;
      }
      if (const auto *dsme =
              R.Nodes.getNodeAs<clang::CXXDependentScopeMemberExpr>("dsme")) {
        if (dsme->isArrow()) {
          clang::MemberExpr *expr = lookupArrowAccess(
              func->getDescribedFunctionTemplate(), dsme->getMemberNameInfo(),
              dsme->getQualifierLoc());
          add(Mapper::ToString(expr));
          return;
        }
        clang::NamedDecl *decl = lookupMemberAccess(
            func->getDescribedFunctionTemplate(), dsme->getMember());
        add(Mapper::ToString(decl));
        return;
      }
      if (const auto *uctor =
              R.Nodes.getNodeAs<clang::CXXUnresolvedConstructExpr>("uctor")) {
        LookupInfo lookup(uctor);
        clang::NamedDecl *decl =
            lookupCalledDecl(func->getDescribedFunctionTemplate(), lookup);
        add(Mapper::ToString(decl));
        return;
      }
      if (const auto *lit =
              R.Nodes.getNodeAs<clang::IntegerLiteral>("macro_int")) {
        if (lit->getBeginLoc().isMacroID()) {
          add(Mapper::ToString(lit));
        }
        return;
      }
    }
  }

private:
  llvm::json::Object &out_;
  clang::Sema *sema_ = nullptr;
  clang::SourceLocation loc_;

  void forceCompleteDefinition(clang::QualType type) {
    type = type.getCanonicalType();
    if (type->isPointerType()) {
      type = type->getPointeeType();
    }

    if (!type->isIncompleteType()) {
      return;
    }

    sema_->RequireCompleteType(loc_, type,
                               clang::Sema::CompleteTypeKind::Normal,
                               clang::diag::err_incomplete_type);

    if (auto *spec =
            llvm::dyn_cast_or_null<clang::ClassTemplateSpecializationDecl>(
                type->getAsCXXRecordDecl())) {
      for (const auto *decl : spec->decls()) {
        if (const auto *tdef = llvm::dyn_cast<clang::TypedefNameDecl>(decl)) {
          clang::QualType tdef_t = tdef->getUnderlyingType();
          forceCompleteDefinition(tdef_t);
        }
      }

      for (const auto &arg : spec->getTemplateArgs().asArray()) {
        if (arg.getKind() == clang::TemplateArgument::Type) {
          forceCompleteDefinition(arg.getAsType());
        }
      }
    }
  }

  clang::FunctionDecl *deduceTemplateArguments(
      clang::FunctionTemplateDecl *decl, llvm::ArrayRef<clang::Expr *> callArgs,
      clang::QualType obj_t, clang::Expr::Classification exprClass,
      clang::TemplateArgumentListInfo *explicitArgs = nullptr) {
    clang::FunctionDecl *spec = nullptr;
    clang::sema::TemplateDeductionInfo info((loc_));
    auto check = [](llvm::ArrayRef<clang::QualType>, bool) -> bool {
      return false;
    };

    auto result = sema_->DeduceTemplateArguments(
        decl, explicitArgs, callArgs, spec, info, false, false, false, obj_t,
        exprClass, false, check);

    if (result == clang::TemplateDeductionResult::Success) {
      return spec;
    }

    if (result == clang::TemplateDeductionResult::SubstitutionFailure) {
      if (const auto *deduced = info.takeCanonical()) {
        clang::TemplateArgumentListInfo targsInfo;
        for (const auto &arg : deduced->asArray()) {
          targsInfo.addArgument(
              sema_->getTrivialTemplateArgumentLoc(arg, {}, loc_));
        }

        clang::DefaultArguments defaultArgs;
        clang::Sema::CheckTemplateArgumentInfo ctai;
        auto invalid = sema_->CheckTemplateArgumentList(
            decl, decl->getTemplateParameters(), loc_, targsInfo, defaultArgs,
            true, ctai);

        if (!invalid) {
          return sema_->InstantiateFunctionDeclaration(decl, deduced, loc_);
        }
      }
    }

    return nullptr;
  }

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

  clang::VarDecl *createVarDecl(clang::QualType type, llvm::StringRef name,
                                clang::StorageClass sclass = clang::SC_None) {
    clang::ASTContext &ctx = sema_->Context;
    clang::VarDecl *decl = clang::VarDecl::Create(
        ctx, sema_->CurContext, loc_, loc_, &ctx.Idents.get(name),
        type.getNonReferenceType(), nullptr, sclass);
    sema_->CurContext->addDecl(decl);
    decl->markUsed(ctx);
    return decl;
  }

  clang::DeclRefExpr *createDeclRefExpr(clang::VarDecl *decl) {
    const clang::DeclarationNameInfo nameInfo(decl->getDeclName(), loc_);
    return sema_->BuildDeclRefExpr(decl, decl->getType(), clang::VK_LValue,
                                   nameInfo, decl->getQualifierLoc());
  }

  clang::DeclRefExpr *createConstexprDeclRefExpr(clang::QualType type,
                                                 llvm::StringRef name) {
    clang::VarDecl *decl = createVarDecl(type, name, clang::SC_Static);
    decl->setConstexpr(true);

    clang::Expr *init;
    clang::ASTContext &ctx = sema_->Context;
    if (type->isIntegerType()) {
      init = clang::IntegerLiteral::Create(
          ctx, llvm::APInt(ctx.getIntWidth(type), 1), type, loc_);
    } else {
      init = new (ctx) clang::ImplicitValueInitExpr(type);
    }
    decl->setInit(init);
    return createDeclRefExpr(decl);
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
      if (llvm::isa<clang::TemplateTypeParmDecl>(param)) {
        clang::RecordDecl *rdecl = createRecordDecl(param->getName());
        clang::QualType type =
            sema_->Context.getTagType(clang::ElaboratedTypeKeyword::None,
                                      rdecl->getQualifier(), rdecl, false);
        out.emplace_back(type);
      } else if (const auto *nttp =
                     llvm::dyn_cast<clang::NonTypeTemplateParmDecl>(param)) {
        clang::QualType type = nttp->getType();
        clang::DeclRefExpr *var =
            createConstexprDeclRefExpr(type, param->getName());
        out.emplace_back(var, true);
      } else {
        assert(0 && "Unsupported template param kind");
      }
    }
  }

  clang::FunctionDecl *instantiateRuleDecl(clang::FunctionTemplateDecl *decl) {
    llvm::SmallVector<clang::TemplateArgument, 8> args;
    createTemplateArguments(decl, args);
    return sema_->InstantiateFunctionDeclaration(
        decl, clang::TemplateArgumentList::CreateCopy(sema_->Context, args),
        loc_);
  }

  clang::FunctionDecl *createCandidate(
      clang::NamedDecl *decl, llvm::ArrayRef<clang::Expr *> callArgs,
      clang::TemplateArgumentListInfo *explicitArgs = nullptr,
      clang::QualType obj_t = clang::QualType(),
      clang::Expr::Classification eclass = clang::Expr::Classification()) {
    if (auto *tdecl = llvm::dyn_cast<clang::FunctionTemplateDecl>(decl)) {
      if (auto *fdecl = deduceTemplateArguments(tdecl, callArgs, obj_t, eclass,
                                                explicitArgs)) {
        return fdecl;
      }
      return nullptr;
    }
    return llvm::dyn_cast<clang::FunctionDecl>(decl);
  }

  clang::CXXRecordDecl *resolveCXXRecordDecl(clang::QualType obj_t) {
    obj_t = obj_t.getCanonicalType();
    while (obj_t->isPointerOrReferenceType()) {
      obj_t = obj_t->getPointeeType();
    }

    forceCompleteDefinition(obj_t);
    if (auto *rdecl = obj_t->getAsCXXRecordDecl()) {
      return rdecl->getDefinition();
    }
    return nullptr;
  }

  void regularNameLookup(llvm::ArrayRef<clang::Expr *> callArgs,
                         clang::TemplateArgumentListInfo *explicitTArgs,
                         clang::DeclarationName &name,
                         clang::OverloadCandidateSet &candidates) {
    clang::LookupResult decls(*sema_, name, loc_,
                              clang::Sema::LookupOrdinaryName);
    sema_->LookupQualifiedName(decls, sema_->getStdNamespace());
    for (auto *ndecl : decls) {
      if (auto *candidate = createCandidate(ndecl, callArgs, explicitTArgs)) {
        sema_->AddOverloadCandidate(
            candidate, clang::DeclAccessPair::make(candidate, clang::AS_public),
            callArgs, candidates, false);
      }
    }

    for (const auto *arg : callArgs) {
      if (auto *rdecl = resolveCXXRecordDecl(arg->getType())) {
        for (auto *frdecl : rdecl->friends()) {
          auto *fd = frdecl->getFriendDecl();
          if (!fd) {
            continue;
          }

          if (auto *ndecl = llvm::dyn_cast<clang::NamedDecl>(fd);
              ndecl && ndecl->getDeclName() == name) {
            if (auto *candidate =
                    createCandidate(ndecl, callArgs, explicitTArgs)) {
              sema_->AddOverloadCandidate(
                  candidate,
                  clang::DeclAccessPair::make(candidate, clang::AS_public),
                  callArgs, candidates, false);
            }
          }
        }
      }
    }
  }

  void cxxMethodNameLookup(clang::QualType obj_t,
                           llvm::ArrayRef<clang::Expr *> callArgs,
                           clang::TemplateArgumentListInfo *explicitTArgs,
                           clang::DeclarationName &name,
                           clang::OverloadCandidateSet &candidates) {
    clang::CXXRecordDecl *rdecl = resolveCXXRecordDecl(obj_t);
    assert(rdecl && "Failed fetching record decl");
    clang::LookupResult members(*sema_, name, loc_,
                                clang::Sema::LookupMemberName);
    sema_->LookupQualifiedName(members, rdecl);

    auto eclass = clang::Expr::Classification::makeSimpleLValue();
    for (auto *ndecl : members) {
      if (auto *candidate =
              createCandidate(ndecl, callArgs, explicitTArgs, obj_t, eclass)) {
        sema_->AddMethodCandidate(
            clang::DeclAccessPair::make(candidate, clang::AS_public), obj_t,
            eclass, callArgs, candidates);
      }
    }
  }

  void cxxConstructorNameLookup(clang::QualType obj_t,
                                llvm::ArrayRef<clang::Expr *> callArgs,
                                clang::OverloadCandidateSet &candidates) {
    clang::CXXRecordDecl *rdecl = resolveCXXRecordDecl(obj_t);
    assert(rdecl && "Failed fetching record decl");
    clang::DeclContextLookupResult ctors = sema_->LookupConstructors(rdecl);

    for (auto *ndecl : ctors) {
      if (auto *candidate = createCandidate(ndecl, callArgs)) {
        sema_->AddOverloadCandidate(
            candidate, clang::DeclAccessPair::make(candidate, clang::AS_public),
            callArgs, candidates, false);
      }
    }
  }

  void adlLookup(llvm::ArrayRef<clang::Expr *> callArgs,
                 clang::DeclarationName &name,
                 clang::OverloadCandidateSet &candidates) {
    clang::ADLResult adl;
    sema_->ArgumentDependentLookup(name, loc_, callArgs, adl);

    for (auto *ndecl : adl) {
      if (auto *candidate = createCandidate(ndecl, callArgs)) {
        sema_->AddOverloadCandidate(
            candidate, clang::DeclAccessPair::make(candidate, clang::AS_public),
            callArgs, candidates, false);
      }
    }
  }

  clang::FunctionDecl *lookupCalledDecl(clang::FunctionTemplateDecl *decl,
                                        LookupInfo &lookup) {
    clang::NamespaceDecl *ns = createNamespaceDecl();
    clang::Sema::ContextRAII savedContext(*sema_, ns);
    clang::FunctionDecl *rule = instantiateRuleDecl(decl);
    llvm::ArrayRef<clang::ParmVarDecl *> parms = rule->parameters();
    auto csk = lookup.name.getNameKind() ==
                       clang::DeclarationName::NameKind::CXXOperatorName
                   ? clang::OverloadCandidateSet::CSK_Operator
                   : clang::OverloadCandidateSet::CSK_Normal;

    llvm::SmallVector<clang::Expr *, 8> callArgs;
    for (const auto *parm : parms) {
      clang::QualType parm_t = parm->getType();
      forceCompleteDefinition(parm_t);
      callArgs.emplace_back(createOpaqueValueExpr(parm_t));
    }

    llvm::ArrayRef<clang::TemplateArgument> ruleTArgs =
        rule->getTemplateSpecializationArgs()->asArray();
    clang::TemplateArgumentListInfo explicitTArgs;

    {
      clang::Sema::InstantiatingTemplate Inst(*sema_, loc_, decl);
      assert(!Inst.isInvalid() && "Invalid instantiation context");
      for (const auto &argloc : lookup.explicitArgs) {
        const auto &arg = argloc.getArgument();
        if (!arg.isDependent()) {
          explicitTArgs.addArgument(argloc);
          continue;
        }

        clang::TemplateArgument inst;
        if (arg.getKind() == clang::TemplateArgument::Type) {
          clang::QualType type = arg.getAsType();
          clang::MultiLevelTemplateArgumentList mtal;
          mtal.setKind(clang::TemplateSubstitutionKind::Rewrite);
          mtal.addOuterTemplateArguments(ruleTArgs);

          clang::TypeSourceInfo *tsi =
              sema_->SubstType(sema_->Context.getTrivialTypeSourceInfo(type),
                               mtal, loc_, clang::DeclarationName());
          assert(tsi && "Template argument type instantiation failed");
          inst = clang::TemplateArgument(tsi->getType());
        } else if (arg.getKind() == clang::TemplateArgument::Expression) {
          if (auto *expr =
                  llvm::dyn_cast<clang::DeclRefExpr>(arg.getAsExpr())) {
            const auto *nttp =
                llvm::dyn_cast<clang::NonTypeTemplateParmDecl>(expr->getDecl());
            assert(nttp && "Unexpected decl in expr");
            inst = ruleTArgs[nttp->getIndex()];
          } else {
            assert(0 && "Unsupported explicit template argument expression");
          }
        } else {
          assert(0 && "Unsupported explicit template argument kind");
        }

        explicitTArgs.addArgument(
            sema_->getTrivialTemplateArgumentLoc(inst, {}, loc_));
      }
    }

    clang::DeclarationName &name = lookup.name;
    clang::OverloadCandidateSet candidates(loc_, csk);
    switch (lookup.kind) {
    case LookupKind::RegularName:
      regularNameLookup(callArgs, &explicitTArgs, name, candidates);
      break;
    case LookupKind::CXXMethodName: {
      llvm::ArrayRef<clang::Expr *> margs = callArgs;
      cxxMethodNameLookup(margs.front()->getType().getNonReferenceType(),
                          margs.drop_front(), &explicitTArgs, name, candidates);
      break;
    }
    case LookupKind::CXXConstructorName:
      cxxConstructorNameLookup(rule->getReturnType(), callArgs, candidates);
      break;
    case LookupKind::ADL:
      adlLookup(callArgs, name, candidates);
      break;
    }

    clang::OverloadCandidateSet::iterator best;
    switch (candidates.BestViableFunction(*sema_, loc_, best)) {
    case clang::OverloadingResult::OR_Success:
      return best->Function;
    case clang::OverloadingResult::OR_Ambiguous:
      for (auto &candidate : candidates) {
        if (candidate.Viable) {
          return candidate.Function;
        }
      }
      break;
    case clang::OverloadingResult::OR_No_Viable_Function:
      llvm::errs() << "No viable function\n";
      break;
    case clang::OverloadingResult::OR_Deleted:
      llvm::errs() << "Deleted function selected\n";
      break;
    }

    assert(0 && "Rule resolution failed");
    return nullptr;
  }

  clang::NamedDecl *lookupMemberAccess(clang::FunctionTemplateDecl *decl,
                                       clang::DeclarationName name) {
    clang::NamespaceDecl *ns = createNamespaceDecl();
    clang::Sema::ContextRAII savedContext(*sema_, ns);
    clang::FunctionDecl *rule = instantiateRuleDecl(decl);
    clang::CXXRecordDecl *rdecl =
        resolveCXXRecordDecl(rule->getParamDecl(0)->getType());
    assert(rdecl && "Failed fetching record decl");
    clang::LookupResult members(*sema_, name, loc_,
                                clang::Sema::LookupMemberName);
    sema_->LookupQualifiedName(members, rdecl);
    assert(!members.empty() && "Rule resolution failed");
    return members.getRepresentativeDecl();
  }

  clang::MemberExpr *
  lookupArrowAccess(clang::FunctionTemplateDecl *decl,
                    const clang::DeclarationNameInfo &nameInfo,
                    clang::NestedNameSpecifierLoc nns) {
    clang::NamespaceDecl *ns = createNamespaceDecl();
    clang::Sema::ContextRAII savedContext(*sema_, ns);
    clang::FunctionDecl *rule = instantiateRuleDecl(decl);

    clang::Expr *obj = createOpaqueValueExpr(
        rule->getParamDecl(0)->getType().getNonReferenceType());
    auto arrow =
        sema_->BuildOverloadedArrowExpr(sema_->getCurScope(), obj, loc_);
    assert(arrow.isUsable() && "Overloaded arrow operator not found");

    auto *base = arrow.getAs<clang::CXXOperatorCallExpr>();
    assert(base && "Unexpected base type");

    clang::CXXRecordDecl *rdecl =
        resolveCXXRecordDecl(base->getType()->getPointeeType());
    assert(rdecl && "Failed fetching record decl");

    clang::LookupResult members(*sema_, nameInfo.getName(), loc_,
                                clang::Sema::LookupMemberName);
    sema_->LookupQualifiedName(members, rdecl);
    for (auto *ndecl : members) {
      if (auto *vdecl = llvm::dyn_cast<clang::ValueDecl>(ndecl)) {
        clang::MemberExpr *access = sema_->BuildMemberExpr(
            base, true, loc_, nns, loc_, vdecl,
            clang::DeclAccessPair::make(vdecl, clang::AS_public), false,
            nameInfo, vdecl->getType(), clang::VK_LValue, clang::OK_Ordinary);
        assert(access && "Rule resolution failed");
        return access;
      }
    }
    assert(0 && "Rule resolution failed");
    return nullptr;
  }

  clang::QualType lookupType(clang::TypeAliasTemplateDecl *decl) {
    clang::NamespaceDecl *ns = createNamespaceDecl();
    clang::Sema::ContextRAII savedContext(*sema_, ns);

    llvm::SmallVector<clang::TemplateArgument, 4> args;
    createTemplateArguments(decl, args);

    clang::MultiLevelTemplateArgumentList mtal;
    mtal.setKind(clang::TemplateSubstitutionKind::Rewrite);
    mtal.addOuterTemplateArguments(args);

    clang::Sema::InstantiatingTemplate TypeInst(*sema_, loc_, decl, args);
    assert(!TypeInst.isInvalid() && "Invalid instantiation context");

    clang::TypeSourceInfo *tsi =
        sema_->SubstType(decl->getTemplatedDecl()->getTypeSourceInfo(), mtal,
                         loc_, clang::DeclarationName());
    assert(tsi && "Rule resolution failed");
    return tsi->getType();
  }
};

class ActionFactory : public clang::tooling::FrontendActionFactory {
public:
  explicit ActionFactory(llvm::json::Object &out) : cb_(out) {
    using namespace clang::ast_matchers;
    finder_.addMatcher(
        returnStmt(
            isExpansionInMainFile(),
            hasReturnValue(ignoringImplicit(ignoringParenImpCasts(anyOf(
                callExpr().bind("fcall"), cxxConstructExpr().bind("ctor"),
                cxxFunctionalCastExpr(has(ignoringImplicit(
                    ignoringParenImpCasts(cxxConstructExpr().bind("ctor"))))),
                memberExpr(hasDeclaration(fieldDecl())).bind("muse"),
                unresolvedMemberExpr().bind("umuse"),
                declRefExpr(to(anyOf(enumConstantDecl().bind("enum_val"),
                                     decl(unless(parmVarDecl())).bind("decl"))))
                    .bind("declref"),
                unaryOperator(hasUnaryOperand(
                                  declRefExpr(to(decl(unless(parmVarDecl()))))))
                    .bind("udeclref"),
                cxxDependentScopeMemberExpr().bind("dsme"),
                cxxUnresolvedConstructExpr().bind("uctor"),
                integerLiteral().bind("macro_int"))))),
            hasAncestor(functionDecl(isDefinition(),
                                     matchesName("(^|::)f[0-9]+$"),
                                     isExpansionInMainFile())
                            .bind("func"))),
        &cb_);

    finder_.addMatcher(
        typeAliasDecl(matchesName("(^|::)t[0-9]+$"), isExpansionInMainFile())
            .bind("tvar"),
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
        DE.setSuppressAllDiagnostics(true);
        DE.setClient(new clang::IgnoringDiagConsumer(), true);
        CB_->init(CI_->getSema());
        AC_->HandleTranslationUnit(ctx);
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

void Extract(const std::filesystem::path &src_path, llvm::json::Object &out) {
  auto flags = getPlatformClangBeginFlags();
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
    SrcFile("file",
            llvm::cl::desc("Path to a rule's src.cpp. ir_src.json is written "
                           "next to it"),
            llvm::cl::value_desc("src.cpp"), llvm::cl::Required,
            llvm::cl::cat(cat));

} // namespace

int main(int argc, char *argv[]) {
  llvm::cl::HideUnrelatedOptions(cat);
  llvm::cl::ParseCommandLineOptions(argc, argv);

  fs::path src = SrcFile.getValue();
  llvm::errs() << "Preprocessing " << src.string() << '\n';
  llvm::json::Object root;
  cpp2rust::Extract(src, root);

  auto out_path = src.parent_path() / "ir_src.json";
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
