// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/expr_matcher.h"

#include <clang/AST/DeclCXX.h>
#include <clang/AST/DeclTemplate.h>
#include <clang/AST/ExprCXX.h>
#include <clang/Basic/OperatorKinds.h>
#include <clang/Lex/Lexer.h>
#include <clang/Sema/EnterExpressionEvaluationContext.h>
#include <clang/Sema/Overload.h>
#include <clang/Sema/Template.h>
#include <clang/Sema/TemplateDeduction.h>
#include <llvm/ADT/STLExtras.h>
#include <llvm/ADT/SmallBitVector.h>

#include <cassert>
#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "converter/rules/decls.h"

namespace cpp2rust::AstMatcher {

namespace {

bool hasSameArgTypes(clang::ASTContext &ctx, const clang::Expr *use,
                     const clang::Expr *returned) {
  auto use_call = llvm::dyn_cast<clang::CallExpr>(RuleDecls::SkipImplicit(use));
  auto rule_call =
      llvm::dyn_cast<clang::CallExpr>(RuleDecls::SkipImplicit(returned));
  if (!use_call || !rule_call) {
    return true;
  }
  if (use_call->getNumArgs() != rule_call->getNumArgs()) {
    return false;
  }
  for (unsigned i = 0; i < use_call->getNumArgs(); ++i) {
    if (!ctx.hasSameUnqualifiedType(use_call->getArg(i)->getType(),
                                    rule_call->getArg(i)->getType())) {
      return false;
    }
  }
  return true;
}

struct MacroUsage {
  std::string name;

  bool operator==(const MacroUsage &) const = default;
};

struct RewrittenOperatorUsage {
  const clang::Decl *decl;
  clang::BinaryOperatorKind op;

  bool operator==(const RewrittenOperatorUsage &) const = default;
};

using ResolvedDecl = std::variant<std::monostate, const clang::Decl *,
                                  MacroUsage, RewrittenOperatorUsage>;

ResolvedDecl getResolvedDecl(clang::ASTContext &ctx, const clang::Expr *expr,
                             bool must_resolve) {
  expr = RuleDecls::SkipImplicit(expr);
  if (llvm::isa<clang::IntegerLiteral>(expr) &&
      expr->getBeginLoc().isMacroID()) {
    return MacroUsage{
        clang::Lexer::getImmediateMacroName(
            expr->getBeginLoc(), ctx.getSourceManager(), ctx.getLangOpts())
            .str()};
  }
  if (auto rewritten =
          llvm::dyn_cast<clang::CXXRewrittenBinaryOperator>(expr)) {
    auto inner = getResolvedDecl(ctx, rewritten->getDecomposedForm().InnerBinOp,
                                 must_resolve);
    assert(std::holds_alternative<const clang::Decl *>(inner) &&
           "rewritten operator does not call a declaration");
    return RewrittenOperatorUsage{std::get<const clang::Decl *>(inner),
                                  rewritten->getOperator()};
  }
  if (auto cast = llvm::dyn_cast<clang::ExplicitCastExpr>(expr)) {
    return getResolvedDecl(ctx, cast->getSubExpr(), must_resolve);
  }
  if (auto call = llvm::dyn_cast<clang::CallExpr>(expr)) {
    if (auto callee = call->getDirectCallee()) {
      return callee->getCanonicalDecl();
    }
  } else if (auto construct = llvm::dyn_cast<clang::CXXConstructExpr>(expr)) {
    return construct->getConstructor()->getCanonicalDecl();
  } else if (auto member = llvm::dyn_cast<clang::MemberExpr>(expr)) {
    return member->getMemberDecl()->getCanonicalDecl();
  } else if (auto ref = llvm::dyn_cast<clang::DeclRefExpr>(expr)) {
    return ref->getDecl()->getCanonicalDecl();
  }
  if (must_resolve) {
    expr->dump(llvm::errs(), ctx);
  }
  assert(!must_resolve && "unhandled declaration in rule body");
  return std::monostate();
}

// Receivers are references in the rules, dereference the pointer to get a
// reference.
clang::Expr *getObject(clang::ASTContext &ctx, clang::Expr *object) {
  auto type = object->getType();
  if (type->isPointerType()) {
    type = type->getPointeeType();
  } else if (object->isLValue()) {
    return object;
  }
  return new (ctx)
      clang::OpaqueValueExpr(object->getBeginLoc(), type, clang::VK_LValue);
}

std::vector<clang::Expr *> getOperands(clang::ASTContext &ctx,
                                       const clang::Expr *use) {
  auto expr = const_cast<clang::Expr *>(RuleDecls::SkipImplicit(use));
  while (auto cast = llvm::dyn_cast<clang::ExplicitCastExpr>(expr)) {
    expr =
        const_cast<clang::Expr *>(RuleDecls::SkipImplicit(cast->getSubExpr()));
  }
  std::vector<clang::Expr *> operands;
  if (auto rewritten =
          llvm::dyn_cast<clang::CXXRewrittenBinaryOperator>(expr)) {
    auto decomposed = rewritten->getDecomposedForm();
    operands = {const_cast<clang::Expr *>(decomposed.LHS),
                const_cast<clang::Expr *>(decomposed.RHS)};
  } else if (auto op = llvm::dyn_cast<clang::CXXOperatorCallExpr>(expr)) {
    operands.assign(op->arg_begin(), op->arg_end());
    if (!operands.empty() &&
        llvm::isa_and_nonnull<clang::CXXMethodDecl>(op->getDirectCallee())) {
      operands[0] = operands[0]->IgnoreParenImpCasts();
    }
  } else if (auto call = llvm::dyn_cast<clang::CXXMemberCallExpr>(expr)) {
    operands.push_back(getObject(
        ctx, call->getImplicitObjectArgument()->IgnoreParenImpCasts()));
    operands.insert(operands.end(), call->arg_begin(), call->arg_end());
  } else if (auto call = llvm::dyn_cast<clang::CallExpr>(expr)) {
    operands.insert(operands.end(), call->arg_begin(), call->arg_end());
  } else if (auto construct = llvm::dyn_cast<clang::CXXConstructExpr>(expr)) {
    operands.assign(construct->arg_begin(), construct->arg_end());
  } else if (auto member = llvm::dyn_cast<clang::MemberExpr>(expr)) {
    auto base = member->getBase()->IgnoreParenImpCasts();
    if (auto arrow = llvm::dyn_cast<clang::CXXOperatorCallExpr>(base);
        arrow && arrow->getOperator() == clang::OO_Arrow) {
      base = arrow->getArg(0);
    }
    operands.push_back(getObject(ctx, base));
  }
  while (!operands.empty() &&
         llvm::isa<clang::CXXDefaultArgExpr>(operands.back())) {
    operands.pop_back();
  }
  for (auto &operand : operands) {
    auto literal = llvm::dyn_cast<clang::CompoundLiteralExpr>(operand);
    if (!literal) {
      continue;
    }
    auto record = literal->getType()->getAsRecordDecl();
    if (!record || !record->hasAttr<clang::TransparentUnionAttr>()) {
      continue;
    }
    auto init = llvm::cast<clang::InitListExpr>(literal->getInitializer());
    assert(init->getNumInits() == 1);
    operand = init->getInit(0);
  }
  return operands;
}

clang::FunctionDecl *deduce(clang::Sema &sema,
                            clang::FunctionTemplateDecl *rule,
                            clang::TemplateArgumentListInfo &explicit_args,
                            llvm::ArrayRef<clang::Expr *> operands) {
  RuleDecls::Trial trial(sema);
  clang::FunctionDecl *specialization = nullptr;
  clang::sema::TemplateDeductionInfo info(rule->getLocation());
  auto result = sema.DeduceTemplateArguments(
      rule, explicit_args.size() ? &explicit_args : nullptr, operands,
      specialization, info, false, false, false, clang::QualType(),
      clang::Expr::Classification(), false,
      [](llvm::ArrayRef<clang::QualType>, bool) { return false; });
  return result == clang::TemplateDeductionResult::Success && !trial.failed()
             ? specialization
             : nullptr;
}

const clang::Expr *
substituteSpecializationInRuleTmpl(clang::Sema &sema,
                                   clang::FunctionTemplateDecl *rule,
                                   clang::FunctionDecl *specialization) {
  if (!specialization) {
    return nullptr;
  }
  RuleDecls::Trial trial(sema);
  auto pattern = rule->getTemplatedDecl();
  clang::EnterExpressionEvaluationContext unevaluated(
      sema, clang::Sema::ExpressionEvaluationContext::Unevaluated);
  clang::Sema::InstantiatingTemplate instantiating(sema, rule->getLocation(),
                                                   specialization);
  clang::Sema::ContextRAII context(sema, specialization);
  clang::LocalInstantiationScope scope(sema);
  for (unsigned i = 0, j = 0; i < pattern->getNumParams(); ++i) {
    auto param = pattern->getParamDecl(i);
    if (!param->isParameterPack()) {
      scope.InstantiatedLocal(param, specialization->getParamDecl(j++));
      continue;
    }
    scope.MakeInstantiatedLocalArgPack(param);
    for (; j < specialization->getNumParams(); ++j) {
      scope.InstantiatedLocalPackArg(param, specialization->getParamDecl(j));
    }
  }
  auto result =
      sema.SubstExpr(const_cast<clang::Expr *>(RuleDecls::GetReturned(pattern)),
                     sema.getTemplateInstantiationArgs(specialization));
  return result.isInvalid() || trial.failed() ? nullptr : result.get();
}

clang::TemplateArgumentListInfo getResultArgs(clang::Sema &sema,
                                              clang::FunctionTemplateDecl *rule,
                                              const clang::Expr *use) {
  auto loc = rule->getLocation();
  clang::TemplateArgumentListInfo explicit_args(loc, loc);
  llvm::SmallBitVector deducible;
  sema.MarkDeducedTemplateParameters(rule, deducible);
  if (deducible.all()) {
    return explicit_args;
  }
  auto params = rule->getTemplateParameters();
  auto deduced = RuleDecls::DeduceFromType(
      sema, params, rule->getTemplatedDecl()->getReturnType(),
      RuleDecls::SkipImplicit(use)
          ->getType()
          .getCanonicalType()
          .getUnqualifiedType());
  if (!deduced) {
    return explicit_args;
  }
  auto last = static_cast<unsigned>(deducible.find_last_unset());
  for (unsigned i = 0; i <= last; ++i) {
    auto param = params->getParam(i);
    if (param->isParameterPack() || (*deduced)[i].isNull()) {
      break;
    }
    clang::QualType nttp_type;
    if (auto nttp = llvm::dyn_cast<clang::NonTypeTemplateParmDecl>(param)) {
      nttp_type = nttp->getType();
    }
    explicit_args.addArgument(
        sema.getTrivialTemplateArgumentLoc((*deduced)[i], nttp_type, loc));
  }
  return explicit_args;
}

void checkCallable(clang::Sema &sema, const clang::Expr *use,
                   clang::FunctionDecl *rule) {
  auto ref = llvm::dyn_cast<clang::DeclRefExpr>(RuleDecls::SkipImplicit(use));
  if (ref && llvm::isa<clang::FunctionDecl>(ref->getDecl())) {
    return;
  }
  RuleDecls::Trial trial(sema);
  auto loc = use->getBeginLoc();
  auto operands = getOperands(sema.Context, use);
  clang::OverloadCandidateSet set(loc, clang::OverloadCandidateSet::CSK_Normal);
  auto found = clang::DeclAccessPair::make(rule, clang::AS_public);
  if (auto tmpl = rule->getDescribedFunctionTemplate()) {
    auto explicit_args = getResultArgs(sema, tmpl, use);
    sema.AddTemplateOverloadCandidate(
        tmpl, found, explicit_args.size() ? &explicit_args : nullptr, operands,
        set);
  } else {
    sema.AddOverloadCandidate(rule, found, operands, set);
  }
  clang::OverloadCandidateSet::iterator best;
  if (set.BestViableFunction(sema, loc, best) != clang::OR_Success) {
    use->dump(llvm::errs(), sema.Context);
    rule->dump(llvm::errs());
    llvm::report_fatal_error("matched rule is not callable with the use");
  }
}

using MatchResult = std::optional<llvm::ArrayRef<clang::TemplateArgument>>;

MatchResult matchVar(clang::ASTContext &ctx, const ResolvedDecl &usage_decl,
                     const clang::VarDecl *rule) {
  if (getResolvedDecl(ctx, rule->getInit(), /*must_resolve=*/true) !=
      usage_decl) {
    return std::nullopt;
  }
  return llvm::ArrayRef<clang::TemplateArgument>();
}

MatchResult matchFunction(clang::Sema &sema, const clang::Expr *use,
                          const ResolvedDecl &usage_decl,
                          clang::FunctionDecl *rule) {
  auto &ctx = sema.Context;
  auto returned = RuleDecls::GetReturned(rule);
  if (getResolvedDecl(ctx, returned, /*must_resolve=*/true) != usage_decl ||
      !hasSameArgTypes(ctx, use, returned)) {
    return std::nullopt;
  }
  checkCallable(sema, use, rule);
  return llvm::ArrayRef<clang::TemplateArgument>();
}

MatchResult matchTemplate(clang::Sema &sema, const clang::Expr *use,
                          const ResolvedDecl &usage_decl,
                          clang::FunctionTemplateDecl *rule) {
  auto &ctx = sema.Context;
  auto explicit_args = getResultArgs(sema, rule, use);
  auto specialization =
      deduce(sema, rule, explicit_args, getOperands(ctx, use));
  auto returned =
      substituteSpecializationInRuleTmpl(sema, rule, specialization);
  if (!returned ||
      getResolvedDecl(ctx, returned, /*must_resolve=*/true) != usage_decl) {
    return std::nullopt;
  }
  checkCallable(sema, use, rule->getTemplatedDecl());
  return specialization->getTemplateSpecializationArgs()->asArray();
}

} // namespace

MatchResult Match(clang::Sema &sema, const clang::Expr *use,
                  const clang::NamedDecl *rule_decl) {
  if (use->isInstantiationDependent()) {
    return std::nullopt;
  }
  auto usage_decl = getResolvedDecl(sema.Context, use, /*must_resolve=*/false);
  if (std::holds_alternative<std::monostate>(usage_decl)) {
    return std::nullopt;
  }
  if (auto var = llvm::dyn_cast<clang::VarDecl>(rule_decl)) {
    return matchVar(sema.Context, usage_decl, var);
  }
  auto rule = const_cast<clang::NamedDecl *>(rule_decl)->getAsFunction();
  if (auto tmpl = rule->getDescribedFunctionTemplate()) {
    return matchTemplate(sema, use, usage_decl, tmpl);
  }
  return matchFunction(sema, use, usage_decl, rule);
}

const clang::NamedDecl *MoreSpecialized(clang::Sema &sema,
                                        const clang::Expr *use,
                                        const clang::NamedDecl *a,
                                        const clang::NamedDecl *b) {
  auto a_function = a->getAsFunction();
  auto b_function = b->getAsFunction();
  auto a_tmpl =
      a_function ? a_function->getDescribedFunctionTemplate() : nullptr;
  auto b_tmpl =
      b_function ? b_function->getDescribedFunctionTemplate() : nullptr;
  if (!a_tmpl || !b_tmpl) {
    if (!a_tmpl && b_tmpl) {
      return a;
    }
    return a_tmpl && !b_tmpl ? b : nullptr;
  }
  RuleDecls::Trial trial(sema);
  auto better = sema.getMoreSpecializedTemplate(
      a_tmpl, b_tmpl, use->getBeginLoc(), clang::TPOC_Call,
      getOperands(sema.Context, use).size());
  if (!better) {
    better = sema.getMoreSpecializedTemplate(a_tmpl, b_tmpl, use->getBeginLoc(),
                                             clang::TPOC_Conversion, 0);
  }
  if (better == a_tmpl) {
    return a;
  }
  return better == b_tmpl ? b : nullptr;
}

} // namespace cpp2rust::AstMatcher
