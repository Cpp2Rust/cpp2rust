// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/translation_rule.h"

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
#include <llvm/Support/JSON.h>
#include <llvm/Support/MemoryBuffer.h>

#include <algorithm>
#include <fstream>
#include <iterator>
#include <regex>
#include <string>
#include <vector>

#include "compat/platform_flags.h"
#include "converter/mapper.h"

namespace cpp2rust::TranslationRule {

namespace {

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

template <typename Vec>
concept TemplateArgVec = requires(Vec vec, std::size_t n) {
  requires std::same_as<typename Vec::value_type, clang::TemplateArgument>;

  { vec.resize(n) };
  { vec.size() } -> std::convertible_to<std::size_t>;
  { vec.operator[](n) } -> std::same_as<clang::TemplateArgument &>;
};

template <TemplateArgVec Container>
class ArgListBuilder
    : public clang::RecursiveASTVisitor<ArgListBuilder<Container>> {

public:
  ArgListBuilder(
      llvm::SmallVectorImpl<Container> &out,
      llvm::SmallDenseMap<clang::DeclarationName, clang::TemplateArgument> &map)
      : out_(&out), map_(&map) {}

  bool VisitTemplateTypeParmType(clang::TemplateTypeParmType *type) {
    const auto *decl = type->getDecl();
    unsigned depth = decl->getDepth();
    unsigned idx = decl->getIndex();

    if (depth >= out_->size()) {
      out_->resize(depth + 1);
    }
    auto &level = (*out_)[depth];
    if (idx >= level.size()) {
      level.resize(idx + 1);
    }

    level[idx] = map_->at(decl->getDeclName());
    return true;
  }

  bool VisitDeclRefExpr(clang::DeclRefExpr *expr) {
    const auto *decl =
        llvm::dyn_cast<clang::NonTypeTemplateParmDecl>(expr->getDecl());
    assert(decl && "Unexpected decl in type");

    unsigned depth = decl->getDepth();
    unsigned idx = decl->getIndex();

    if (depth >= out_->size()) {
      out_->resize(depth + 1);
    }
    auto &level = (*out_)[depth];
    if (idx >= level.size()) {
      level.resize(idx + 1);
    }

    level[idx] = map_->at(decl->getDeclName());
    return true;
  }

private:
  llvm::SmallVectorImpl<Container> *out_;
  llvm::SmallDenseMap<clang::DeclarationName, clang::TemplateArgument> *map_;
};

using RuleMap = std::unordered_map<std::string, Rule>;

class Callback : public clang::ast_matchers::MatchFinder::MatchCallback {
public:
  explicit Callback(RuleMap &out) : out_(out) {}

  void setSema(clang::Sema &sema) { sema_ = &sema; }

  void run(const clang::ast_matchers::MatchFinder::MatchResult &R) override {
    assert(sema_);
    if (auto func = R.Nodes.getNodeAs<clang::FunctionDecl>("func")) {
      auto sym = func->getQualifiedNameAsString();
      auto add = [&](std::string src) {
        Rule rule;
        rule.src = std::move(src);
        out_[sym] = std::move(rule);
      };

      if (auto var = R.Nodes.getNodeAs<clang::ParmVarDecl>("tvar")) {
        clang::QualType type;
        if (auto *tdecl = func->getDescribedFunctionTemplate()) {
          type = lookupType(tdecl);
        } else {
          type = var->getType();
        }

        if (const auto *decayed = type->getAs<clang::DecayedType>()) {
          clang::QualType original = decayed->getOriginalType();
          if (original->isArrayType()) {
            type = original;
          }
        }

        add(Mapper::ToString(type));
        return;
      }
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
    }
  }

private:
  RuleMap &out_;
  clang::Sema *sema_ = nullptr;

  void forceCompleteDefinition(clang::QualType type) {
    type = type.getCanonicalType();
    if (type->isPointerType()) {
      type = type->getPointeeType();
    }

    if (!type->isIncompleteType()) {
      return;
    }

    sema_->RequireCompleteType(clang::SourceLocation(), type,
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
      clang::FunctionTemplateDecl *decl,
      llvm::ArrayRef<clang::QualType> instantiatedParms, clang::QualType obj_t,
      clang::Expr::Classification exprClass,
      clang::TemplateArgumentListInfo *explicitArgs = nullptr) {
    llvm::SmallVector<clang::Expr *, 8> args;
    for (const auto &parm_t : instantiatedParms) {
      args.emplace_back(createOpaqueValueExpr(parm_t));
    }

    clang::FunctionDecl *spec = nullptr;
    clang::sema::TemplateDeductionInfo info((clang::SourceLocation()));
    auto check = [](llvm::ArrayRef<clang::QualType>, bool) -> bool {
      return false;
    };

    auto result = sema_->DeduceTemplateArguments(
        decl, explicitArgs, args, spec, info, false, false, false, obj_t,
        exprClass, false, check);

    if (result == clang::TemplateDeductionResult::Success) {
      return spec;
    }

    if (result == clang::TemplateDeductionResult::SubstitutionFailure) {
      if (const auto *deduced = info.takeCanonical()) {
        clang::TemplateArgumentListInfo targsInfo;
        for (const auto &arg : deduced->asArray()) {
          targsInfo.addArgument(
              sema_->getTrivialTemplateArgumentLoc(arg, {}, {}, nullptr));
        }

        clang::DefaultArguments defaultArgs;
        clang::Sema::CheckTemplateArgumentInfo ctai;
        auto invalid = sema_->CheckTemplateArgumentList(
            decl, decl->getTemplateParameters(), clang::SourceLocation(),
            targsInfo, defaultArgs, true, ctai);

        if (!invalid) {
          return sema_->InstantiateFunctionDeclaration(decl, deduced,
                                                       clang::SourceLocation());
        }
      }
    }

    return nullptr;
  }

  clang::NamespaceDecl *createNamespaceDecl() {
    auto &ctx = sema_->getASTContext();
    auto *tu = ctx.getTranslationUnitDecl();
    auto *ns = clang::NamespaceDecl::Create(
        ctx, tu, false, clang::SourceLocation(), clang::SourceLocation(),
        nullptr, nullptr, false);
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
        clang::TagUseKind::Definition, clang::SourceLocation(), scope,
        &sema_->Context.Idents.get(name), clang::SourceLocation(),
        clang::ParsedAttributesView(), clang::AS_none, clang::SourceLocation(),
        args, owned, dependent, clang::SourceLocation(), false,
        clang::TypeResult(), false, false, clang::OffsetOfKind::Outside,
        nullptr);
    assert(decl.isUsable() && "Record decl creation failed");
    auto *rdecl = decl.getAs<clang::RecordDecl>();
    rdecl->startDefinition();
    rdecl->completeDefinition();
    return rdecl;
  }

  clang::VarDecl *createVarDecl(clang::QualType type, llvm::StringRef name,
                                clang::StorageClass sclass = clang::SC_None) {
    clang::ASTContext &ctx = sema_->Context;
    clang::VarDecl *decl =
        clang::VarDecl::Create(ctx, sema_->CurContext, clang::SourceLocation(),
                               clang::SourceLocation(), &ctx.Idents.get(name),
                               type.getNonReferenceType(), nullptr, sclass);
    sema_->CurContext->addDecl(decl);
    decl->markUsed(ctx);
    return decl;
  }

  clang::DeclRefExpr *createDeclRefExpr(clang::VarDecl *decl) {
    return clang::DeclRefExpr::Create(
        sema_->Context, clang::NestedNameSpecifierLoc(),
        clang::SourceLocation(), decl, false, clang::SourceLocation(),
        decl->getType().getCanonicalType(), clang::VK_LValue);
  }

  clang::DeclRefExpr *createConstexprDeclRefExpr(clang::QualType type,
                                                 llvm::StringRef name) {
    clang::VarDecl *decl = createVarDecl(type, name, clang::SC_Static);
    decl->setConstexpr(true);

    clang::Expr *init;
    clang::ASTContext &ctx = sema_->Context;
    if (type->isIntegerType()) {
      init = clang::IntegerLiteral::Create(ctx, llvm::APInt(1, 1), type, {});
    } else {
      init = new (ctx) clang::ImplicitValueInitExpr(type);
    }
    decl->setInit(init);
    return createDeclRefExpr(decl);
  }

  clang::OpaqueValueExpr *createOpaqueValueExpr(clang::QualType type) {
    return new (sema_->Context) clang::OpaqueValueExpr(
        {}, type.getNonReferenceType(),
        type->isRValueReferenceType() ? clang::VK_XValue : clang::VK_LValue);
  }

  clang::FunctionDecl *instantiateRuleDecl(
      clang::FunctionTemplateDecl *decl,
      llvm::SmallDenseMap<clang::DeclarationName, clang::TemplateArgument>
          *argMap = nullptr) {
    clang::ASTContext &ctx = sema_->Context;
    llvm::SmallVector<clang::TemplateArgument, 8> args;

    for (clang::NamedDecl *param : *decl->getTemplateParameters()) {
      if (llvm::isa<clang::TemplateTypeParmDecl>(param)) {
        clang::RecordDecl *rdecl = createRecordDecl(param->getName());
        clang::QualType type = ctx.getTagType(
            clang::ElaboratedTypeKeyword::None, {}, rdecl, false);
        args.emplace_back(type);
        if (argMap) {
          argMap->try_emplace(param->getDeclName(), type);
        }
      } else if (const auto *nttp =
                     llvm::dyn_cast<clang::NonTypeTemplateParmDecl>(param)) {
        clang::QualType type = nttp->getType();
        clang::DeclRefExpr *var =
            createConstexprDeclRefExpr(type, param->getName());
        args.emplace_back(var, true);
        if (argMap) {
          argMap->try_emplace(param->getDeclName(), var, true);
        }
      } else {
        assert(0 && "Unsupported template param kind");
      }
    }

    return sema_->InstantiateFunctionDeclaration(
        decl, clang::TemplateArgumentList::CreateCopy(ctx, args),
        clang::SourceLocation());
  }

  clang::FunctionDecl *createCandidate(
      clang::NamedDecl *decl, llvm::ArrayRef<clang::QualType> parms_t,
      clang::TemplateArgumentListInfo *explicitArgs = nullptr,
      clang::QualType obj_t = clang::QualType(),
      clang::Expr::Classification eclass = clang::Expr::Classification()) {
    if (auto *tdecl = llvm::dyn_cast<clang::FunctionTemplateDecl>(decl)) {
      if (auto *fdecl = deduceTemplateArguments(tdecl, parms_t, obj_t, eclass,
                                                explicitArgs)) {
        return fdecl;
      }
      return nullptr;
    }
    return llvm::dyn_cast<clang::FunctionDecl>(decl);
  }

  clang::CXXRecordDecl *resolveCXXRecordDecl(clang::QualType obj_t) {
    obj_t = obj_t.getCanonicalType();
    if (obj_t->isReferenceType()) {
      obj_t = obj_t.getNonReferenceType();
    }

    forceCompleteDefinition(obj_t);
    if (auto *rdecl = obj_t->getAsCXXRecordDecl()) {
      return rdecl->getDefinition();
    }
    return nullptr;
  }

  void regularNameLookup(llvm::ArrayRef<clang::Expr *> args,
                         llvm::ArrayRef<clang::QualType> parms,
                         clang::TemplateArgumentListInfo *explicitArgs,
                         clang::DeclarationName &name,
                         clang::OverloadCandidateSet &candidates) {
    clang::LookupResult decls(*sema_, name, clang::SourceLocation(),
                              clang::Sema::LookupOrdinaryName);
    sema_->LookupQualifiedName(decls, sema_->getStdNamespace());
    for (auto *ndecl : decls) {
      if (auto *candidate = createCandidate(ndecl, parms, explicitArgs)) {
        sema_->AddOverloadCandidate(
            candidate, clang::DeclAccessPair::make(candidate, clang::AS_public),
            args, candidates, false);
      }
    }

    for (const auto &parm_t : parms) {
      if (auto *rdecl = resolveCXXRecordDecl(parm_t)) {
        for (auto *frdecl : rdecl->friends()) {
          auto *fd = frdecl->getFriendDecl();
          if (!fd) {
            continue;
          }

          if (auto *ndecl = llvm::dyn_cast<clang::NamedDecl>(fd);
              ndecl && ndecl->getDeclName() == name) {
            if (auto *candidate = createCandidate(ndecl, parms, explicitArgs)) {
              sema_->AddOverloadCandidate(
                  candidate,
                  clang::DeclAccessPair::make(candidate, clang::AS_public),
                  args, candidates, false);
            }
          }
        }
      }
    }
  }

  void cxxMethodNameLookup(clang::QualType obj_t,
                           llvm::ArrayRef<clang::Expr *> args,
                           llvm::ArrayRef<clang::QualType> parms,
                           clang::TemplateArgumentListInfo *explicitArgs,
                           clang::DeclarationName &name,
                           clang::OverloadCandidateSet &candidates) {
    clang::CXXRecordDecl *rdecl = resolveCXXRecordDecl(obj_t);
    assert(rdecl && "Failed fetching record decl");
    clang::LookupResult members(*sema_, name, clang::SourceLocation(),
                                clang::Sema::LookupMemberName);
    sema_->LookupQualifiedName(members, rdecl);

    auto eclass = clang::Expr::Classification::makeSimpleLValue();
    for (auto *ndecl : members) {
      if (auto *candidate =
              createCandidate(ndecl, parms, explicitArgs, obj_t, eclass)) {
        sema_->AddMethodCandidate(
            clang::DeclAccessPair::make(candidate, clang::AS_public), obj_t,
            eclass, args, candidates);
      }
    }
  }

  void cxxConstructorNameLookup(clang::QualType obj_t,
                                llvm::ArrayRef<clang::Expr *> args,
                                llvm::ArrayRef<clang::QualType> parms,
                                clang::OverloadCandidateSet &candidates) {
    clang::CXXRecordDecl *rdecl = resolveCXXRecordDecl(obj_t);
    assert(rdecl && "Failed fetching record decl");
    clang::DeclContextLookupResult ctors = sema_->LookupConstructors(rdecl);

    for (auto *ndecl : ctors) {
      if (auto *candidate = createCandidate(ndecl, parms)) {
        sema_->AddOverloadCandidate(
            candidate, clang::DeclAccessPair::make(candidate, clang::AS_public),
            args, candidates, false);
      }
    }
  }

  void adlLookup(llvm::ArrayRef<clang::Expr *> args,
                 llvm::ArrayRef<clang::QualType> parms,
                 clang::DeclarationName &name,
                 clang::OverloadCandidateSet &candidates) {
    clang::ADLResult adl;
    sema_->ArgumentDependentLookup(name, {}, args, adl);

    for (auto *ndecl : adl) {
      if (auto *candidate = createCandidate(ndecl, parms)) {
        sema_->AddOverloadCandidate(
            candidate, clang::DeclAccessPair::make(candidate, clang::AS_public),
            args, candidates, false);
      }
    }
  }

  clang::FunctionDecl *lookupCalledDecl(clang::FunctionTemplateDecl *decl,
                                        LookupInfo &lookup) {
    clang::NamespaceDecl *ns = createNamespaceDecl();
    clang::Sema::ContextRAII savedContext(*sema_, ns);
    llvm::SmallDenseMap<clang::DeclarationName, clang::TemplateArgument> targs;
    clang::FunctionDecl *rule = instantiateRuleDecl(decl, &targs);
    llvm::ArrayRef<clang::ParmVarDecl *> parms = rule->parameters();
    auto csk = lookup.name.getNameKind() ==
                       clang::DeclarationName::NameKind::CXXOperatorName
                   ? clang::OverloadCandidateSet::CSK_Operator
                   : clang::OverloadCandidateSet::CSK_Normal;

    llvm::SmallVector<clang::Expr *, 8> args;
    llvm::SmallVector<clang::QualType, 8> parms_t;
    for (const auto *parm : parms) {
      clang::QualType parm_t = parm->getType();
      forceCompleteDefinition(parm_t);
      args.emplace_back(createOpaqueValueExpr(parm_t));
      parms_t.emplace_back(parm_t);
    }

    clang::TemplateArgumentListInfo explicitArgs;
    for (const auto &argloc : lookup.explicitArgs) {
      const auto &arg = argloc.getArgument();
      if (!arg.isDependent()) {
        explicitArgs.addArgument(argloc);
        continue;
      }

      clang::TemplateArgument inst;
      if (arg.getKind() == clang::TemplateArgument::Type) {
        clang::QualType type = arg.getAsType();
        llvm::SmallVector<llvm::SmallVector<clang::TemplateArgument, 8>, 4>
            replacements;
        ArgListBuilder builder(replacements, targs);
        builder.TraverseType(type);

        clang::MultiLevelTemplateArgumentList mtal;
        mtal.setKind(clang::TemplateSubstitutionKind::Rewrite);
        for (auto &level : replacements) {
          mtal.addOuterTemplateArguments(level);
        }

        clang::TypeSourceInfo *tsi =
            sema_->SubstType(sema_->Context.getTrivialTypeSourceInfo(type),
                             mtal, {}, clang::DeclarationName());
        inst = clang::TemplateArgument(tsi->getType());
      } else if (arg.getKind() == clang::TemplateArgument::Expression) {
        if (auto *decl = llvm::dyn_cast<clang::DeclRefExpr>(arg.getAsExpr())) {
          inst = targs.at(decl->getDecl()->getDeclName());
        } else {
          assert(0 && "Unsupported explicit template argument expression");
        }
      } else {
        assert(0 && "Unsupported explicit template argument kind");
      }

      explicitArgs.addArgument(
          sema_->getTrivialTemplateArgumentLoc(inst, {}, {}, nullptr));
    }

    clang::DeclarationName &name = lookup.name;
    clang::OverloadCandidateSet candidates(clang::SourceLocation(), csk);
    switch (lookup.kind) {
    case LookupKind::RegularName:
      regularNameLookup(args, parms_t, &explicitArgs, name, candidates);
      break;
    case LookupKind::CXXMethodName: {
      llvm::ArrayRef<clang::Expr *> margs = args;
      llvm::ArrayRef<clang::QualType> mparms = parms_t;
      cxxMethodNameLookup(parms_t.front().getNonReferenceType(),
                          margs.drop_front(), mparms.drop_front(),
                          &explicitArgs, name, candidates);
      break;
    }
    case LookupKind::CXXConstructorName:
      cxxConstructorNameLookup(rule->getReturnType(), args, parms_t,
                               candidates);
      break;
    case LookupKind::ADL:
      adlLookup(args, parms_t, name, candidates);
      break;
    }

    clang::OverloadCandidateSet::iterator best;
    switch (
        candidates.BestViableFunction(*sema_, clang::SourceLocation(), best)) {
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
    clang::LookupResult members(*sema_, name, clang::SourceLocation(),
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
    auto arrow = sema_->BuildOverloadedArrowExpr(sema_->getCurScope(), obj,
                                                 clang::SourceLocation());
    assert(arrow.isUsable() && "Overloaded arrow operator not found");

    auto *base = arrow.getAs<clang::CXXOperatorCallExpr>();
    assert(base && "Unexpected base type");

    clang::CXXRecordDecl *rdecl =
        resolveCXXRecordDecl(base->getType()->getPointeeType());
    assert(rdecl && "Failed fetching record decl");

    clang::LookupResult members(*sema_, nameInfo.getName(),
                                clang::SourceLocation(),
                                clang::Sema::LookupMemberName);
    sema_->LookupQualifiedName(members, rdecl);
    for (auto *ndecl : members) {
      if (auto *vdecl = llvm::dyn_cast<clang::ValueDecl>(ndecl)) {
        clang::MemberExpr *access = sema_->BuildMemberExpr(
            base, true, clang::SourceLocation(), nns, clang::SourceLocation(),
            vdecl, clang::DeclAccessPair::make(vdecl, clang::AS_public), false,
            nameInfo, vdecl->getType(), clang::VK_LValue, clang::OK_Ordinary);
        assert(access && "Rule resolution failed");
        return access;
      }
    }
    assert(0 && "Rule resolution failed");
    return nullptr;
  }

  clang::QualType lookupType(clang::FunctionTemplateDecl *decl) {
    clang::NamespaceDecl *ns = createNamespaceDecl();
    clang::Sema::ContextRAII savedContext(*sema_, ns);
    clang::FunctionDecl *rule = instantiateRuleDecl(decl);
    assert(rule && "Rule resolution failed");
    return rule->getParamDecl(0)->getType();
  }
};

class ActionFactory : public clang::tooling::FrontendActionFactory {
public:
  explicit ActionFactory(RuleMap &out) : cb_(out) {
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
                cxxUnresolvedConstructExpr().bind("uctor"))))),
            hasAncestor(functionDecl(isDefinition(),
                                     matchesName("(^|::)f[0-9]+$"),
                                     isExpansionInMainFile())
                            .bind("func"))),
        &cb_);

    finder_.addMatcher(
        parmVarDecl(hasAncestor(functionDecl(matchesName("(^|::)t[0-9]+$"),
                                             isExpansionInMainFile())
                                    .bind("func")),
                    isExpansionInMainFile())
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
        CB_->setSema(CI_->getSema());
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

TextFragment ParseTextFragmentJSON(const llvm::json::Object &obj) {
  return {obj.getString("text")->str()};
}

GenericFragment ParseGenericFragmentJSON(const llvm::json::Object &obj) {
  return {obj.getString("generic")->str()};
}

TypeInfo ParseTypeInfoJSON(const llvm::json::Object &obj) {
  TypeInfo info;
  if (auto ty = obj.getString("type"))
    info.type = ty->str();
  if (auto v = obj.getBoolean("is_refcount_pointer"))
    info.is_refcount_pointer = *v;
  if (auto v = obj.getBoolean("is_unsafe_pointer"))
    info.is_unsafe_pointer = *v;
  assert(!(info.is_refcount_pointer && info.is_unsafe_pointer));
  return info;
}

Access ParseAccessJSON(llvm::StringRef value) {
  if (value == "read") {
    return Access::kRead;
  } else if (value == "write") {
    return Access::kWrite;
  } else if (value == "move") {
    return Access::kMove;
  } else {
    llvm::errs() << "Invalid access value: " << value << '\n';
    assert(0);
    return Access::kRead;
  }
}

PlaceholderFragment
ParsePlaceholderFragmentJSON(const llvm::json::Object &obj) {
  auto arg = obj.getString("arg");
  auto access = obj.getString("access");
  assert(arg && access);
  return {arg->str(), ParseAccessJSON(*access)};
}

std::vector<BodyFragment> ParseBodyFragmentsJSON(const llvm::json::Array &arr);

MethodCallFragment ParseMethodCallFragmentJSON(const llvm::json::Object &obj) {
  MethodCallFragment mc;
  if (auto *receiver = obj.getArray("receiver")) {
    mc.receiver = ParseBodyFragmentsJSON(*receiver);
  }
  if (auto *body = obj.getArray("body")) {
    mc.body = ParseBodyFragmentsJSON(*body);
  }
  return mc;
}

std::vector<BodyFragment> ParseBodyFragmentsJSON(const llvm::json::Array &arr) {
  std::vector<BodyFragment> result;
  for (auto &frag : arr) {
    auto *frag_obj = frag.getAsObject();
    if (!frag_obj)
      continue;
    if (frag_obj->getString("text")) {
      result.push_back(ParseTextFragmentJSON(*frag_obj));
    } else if (frag_obj->getString("generic")) {
      result.push_back(ParseGenericFragmentJSON(*frag_obj));
    } else if (auto *ph = frag_obj->getObject("placeholder")) {
      result.push_back(ParsePlaceholderFragmentJSON(*ph));
    } else if (auto *mc = frag_obj->getObject("method_call")) {
      result.push_back(std::make_unique<MethodCallFragment>(
          ParseMethodCallFragmentJSON(*mc)));
    }
  }
  return result;
}

ExprTgt ParseExprTgtJSON(const llvm::json::Object &obj) {
  ExprTgt ir;

  if (auto *params = obj.getObject("params")) {
    for (auto &[key, val] : *params) {
      if (auto *param_obj = val.getAsObject())
        ir.params[key.str()] = ParseTypeInfoJSON(*param_obj);
    }
  }

  if (auto *rt = obj.getObject("return_type"))
    ir.return_type = ParseTypeInfoJSON(*rt);

  if (auto ms = obj.getBoolean("multi_statement"))
    ir.multi_statement = *ms;

  if (auto *generics = obj.getObject("generics")) {
    for (auto &[key, val] : *generics) {
      if (auto *arr = val.getAsArray()) {
        std::vector<std::string> bounds;
        for (auto &b : *arr) {
          if (auto s = b.getAsString())
            bounds.push_back(s->str());
        }
        ir.generics[key.str()] = std::move(bounds);
      }
    }
  }

  if (auto *body = obj.getArray("body")) {
    ir.body = ParseBodyFragmentsJSON(*body);
  }

  return ir;
}

TypeTgt ParseTypeTgtJSON(const llvm::json::Object &obj) {
  TypeTgt tgt;
  if (auto init = obj.getString("init"))
    tgt.initializer = init->str();
  tgt.type_info = ParseTypeInfoJSON(obj);
  return tgt;
}

RuleMap LoadSrc(const std::filesystem::path &src_path) {
  clang::tooling::FixedCompilationDatabase compilations(
      ".", getPlatformClangFlags());
  RuleMap out;
  ActionFactory factory(out);
  clang::tooling::ClangTool tool(compilations, {src_path.string()});
  tool.run(&factory);

  if (out.empty()) {
    llvm::errs() << "Warning: no symbols found in return statements for file: "
                 << src_path << '\n';
    return out;
  }
  return out;
}

RuleMap LoadTgtFromIR(const std::filesystem::path &json_path) {
  RuleMap out;

  auto buf = llvm::MemoryBuffer::getFile(json_path.string());
  if (!buf)
    return out;

  auto parsed = llvm::json::parse((*buf)->getBuffer());
  if (!parsed) {
    llvm::errs() << "Failed to parse IR JSON: " << json_path << ": "
                 << llvm::toString(parsed.takeError()) << '\n';
    assert(0);
    return out;
  }

  auto *root = parsed->getAsObject();
  if (!root)
    return out;

  for (auto &[entry_name, entry_val] : *root) {
    auto *obj = entry_val.getAsObject();
    if (!obj)
      continue;

    auto name = entry_name.str();
    Rule rule;
    if (name[0] == 'f') {
      rule.tgt = ParseExprTgtJSON(*obj);
      std::get<ExprTgt>(rule.tgt).validate(json_path.string() + ":" + name);
    } else if (name[0] == 't') {
      rule.tgt = ParseTypeTgtJSON(*obj);
    } else {
      continue;
    }
    out[name] = std::move(rule);
  }

  return out;
}

template <typename Map>
void ValidateConsecutiveKeys(const Map &map, char prefix, int start,
                             const std::string &label) {
  std::vector<int> indices;
  for (auto &[key, _] : map) {
    if (key.size() < 2 || key[0] != prefix ||
        !std::all_of(key.begin() + 1, key.end(), ::isdigit)) {
      llvm::errs() << label << ": invalid name '" << key << "', expected "
                   << prefix << start << ", " << prefix << (start + 1) << ", "
                   << prefix << (start + 2) << ", ...\n";
      assert(0 && "names must follow expected pattern");
    }
    indices.push_back(std::stoi(key.substr(1)));
  }
  std::sort(indices.begin(), indices.end());
  for (size_t i = 0; i < indices.size(); ++i) {
    if (indices[i] != static_cast<int>(i) + start) {
      llvm::errs() << label << ": not consecutive. Got:";
      for (auto idx : indices) {
        llvm::errs() << " " << prefix << idx;
      }
      llvm::errs() << '\n';
      assert(0 && "indices must be consecutive");
    }
  }
}

void BodyFragmentDump(const BodyFragment &frag) {
  if (auto *t = std::get_if<TextFragment>(&frag)) {
    t->dump();
  } else if (auto *p = std::get_if<PlaceholderFragment>(&frag)) {
    p->dump();
  } else if (auto *g = std::get_if<GenericFragment>(&frag)) {
    g->dump();
  } else if (auto *mc =
                 std::get_if<std::unique_ptr<MethodCallFragment>>(&frag)) {
    (*mc)->dump();
  }
}

} // namespace

void TextFragment::dump() const {
  llvm::errs() << "  text: \"" << text << "\"\n";
}

void PlaceholderFragment::dump() const {
  llvm::errs() << "  placeholder: " << arg << " (";
  switch (access) {
  case Access::kRead:
    llvm::errs() << "read\n";
    break;
  case Access::kWrite:
    llvm::errs() << "write\n";
    break;
  case Access::kMove:
    llvm::errs() << "move\n";
    break;
  }
}

const PlaceholderFragment *MethodCallFragment::getReceiverPlaceholder() const {
  for (auto &frag : receiver) {
    if (auto *ph = std::get_if<PlaceholderFragment>(&frag)) {
      return ph;
    }
  }
  return nullptr;
}

void MethodCallFragment::dump() const {
  llvm::errs() << "  method_call:\n"
                  "    receiver:\n";
  for (const auto &frag : receiver) {
    BodyFragmentDump(frag);
  }
  llvm::errs() << "    body:\n";
  for (const auto &frag : body) {
    BodyFragmentDump(frag);
  }
}

void ExprTgt::dump() const {
  for (auto &[name, info] : params) {
    llvm::errs() << "  param " << name << ": ";
    info.dump();
    llvm::errs() << '\n';
  }
  if (!return_type.type.empty()) {
    llvm::errs() << "  return: ";
    return_type.dump();
    llvm::errs() << '\n';
  }
  for (auto &[name, bounds] : generics) {
    llvm::errs() << "  generic " << name << ":";
    for (auto &b : bounds) {
      llvm::errs() << " " << b;
    }
    llvm::errs() << '\n';
  }
  for (const auto &frag : body) {
    BodyFragmentDump(frag);
  }
}

void GenericFragment::dump() const {
  llvm::errs() << "  generic: " << name << '\n';
}

void TypeInfo::dump() const {
  llvm::errs() << type;
  if (is_refcount_pointer)
    llvm::errs() << " [rc_ptr]";
  if (is_unsafe_pointer)
    llvm::errs() << " [unsafe_ptr]";
}

void TypeTgt::dump() const {
  llvm::errs() << "  type: ";
  type_info.dump();
  llvm::errs() << '\n';
  if (!initializer.empty()) {
    llvm::errs() << "  init: " << initializer << '\n';
  }
}

void ExprTgt::validate(const std::string &context) const {
  ValidateConsecutiveKeys(params, 'a', 0, context + " params");
  ValidateConsecutiveKeys(generics, 'T', 1, context + " generics");
  assert(!body.empty() && "ExprTgt body must not be empty");
}

std::vector<Rule> Load(const std::filesystem::path &path, Model model) {
  auto dir = path.parent_path();
  auto unsafe_ir_path = dir / "ir_unsafe.json";
  auto refcount_ir_path = dir / "ir_refcount.json";

  auto rules = LoadTgtFromIR(unsafe_ir_path);

  if (model == Model::kRefCount && std::filesystem::exists(refcount_ir_path)) {
    auto refcount = LoadTgtFromIR(refcount_ir_path);
    for (auto &[name, rule] : refcount) {
      rules[name] = std::move(rule);
    }
  }

  auto src_rules = LoadSrc(path);
  if (src_rules.size() == 0) {
    return {};
  }
  for (auto &[name, src_rule] : src_rules) {
    rules.at(name).src = std::move(src_rule.src);
  }

  std::vector<Rule> result;
  for (auto &[name, rule] : rules) {
    if (rule.src.empty()) {
      llvm::errs() << name << '\n';
      if (const auto *tgt = std::get_if<TypeTgt>(&rule.tgt)) {
        tgt->dump();
      }
      if (const auto *tgt = std::get_if<ExprTgt>(&rule.tgt)) {
        tgt->dump();
      }
    }
    assert(!rule.src.empty() && "Rule loaded from IR but has no src");
    result.push_back(std::move(rule));
  }
  return result;
}

} // namespace cpp2rust::TranslationRule
