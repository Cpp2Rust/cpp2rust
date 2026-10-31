// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/registry.h"

#include <clang/AST/Attr.h>
#include <clang/AST/DeclCXX.h>
#include <clang/AST/DeclTemplate.h>
#include <clang/Basic/SourceManager.h>
#include <clang/Sema/Sema.h>
#include <clang/Sema/Template.h>
#include <llvm/ADT/STLExtras.h>
#include <llvm/ADT/StringExtras.h>

#include <algorithm>
#include <cstdlib>
#include <filesystem>
#include <format>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <variant>

#include "converter/converter_lib.h"
#include "converter/rules/decls.h"
#include "converter/rules/expr_matcher.h"
#include "converter/rules/rule_keys.h"
#include "converter/rules/rules_loader.h"
#include "converter/rules/type_matcher.h"

namespace cpp2rust::RuleRegistry {

namespace {

Model model_ = Model::kUnsafe;
clang::Sema *sema_ = nullptr;

std::unordered_multimap<RuleKeys::ExprKey, ExprRuleDecl> exprs_;
std::unordered_multimap<RuleKeys::TypeKey, TypeRuleDecl> types_;

const llvm::json::Object *findRule(Model model, const std::string &dir,
                                   const std::string &name) {
  static std::unordered_map<std::string, llvm::json::Object> indexes;
  auto path =
      std::filesystem::path(RULES_IR_DIR) / dir /
      (model == Model::kRefCount ? "ir_refcount.json" : "ir_unsafe.json");
  auto [it, inserted] = indexes.try_emplace(path.string());
  if (inserted && std::filesystem::exists(path)) {
    it->second = RulesLoader::ReadIndex(path);
  }
  return it->second.getObject(name);
}

bool pointsToUserType(clang::QualType type) {
  if (!type->isPointerType()) {
    return false;
  }
  auto tag = type->getPointeeType()->getAsTagDecl();
  return tag && IsUserDefinedDecl(tag);
}

constexpr llvm::StringLiteral kInitAliasPrefix = "cpp2rust_init_";

bool isRuleName(llvm::StringRef name) {
  return name.size() > 1 && (name[0] == 'f' || name[0] == 't') &&
         llvm::all_of(name.drop_front(), llvm::isDigit);
}

void collectGenerics(const std::vector<TranslationRule::BodyFragment> &body,
                     std::unordered_set<unsigned> &used) {
  TranslationRule::AnyFragment(body, [&](const TranslationRule::BodyFragment
                                             &fragment) {
    if (auto generic =
            std::get_if<TranslationRule::GenericFragment>(&fragment)) {
      used.insert(generic->n);
    }
    if (auto call =
            std::get_if<std::unique_ptr<TranslationRule::GenericCallFragment>>(
                &fragment)) {
      used.insert((*call)->generic);
    }
    return false;
  });
}

void collectGenerics(llvm::StringRef type, std::unordered_set<unsigned> &used) {
  auto is_ident = [](char c) { return llvm::isAlnum(c) || c == '_'; };
  for (size_t i = 0; i < type.size(); ++i) {
    if (type[i] != 'T' || (i > 0 && is_ident(type[i - 1]))) {
      continue;
    }
    size_t end = i + 1;
    while (end < type.size() && llvm::isDigit(type[end])) {
      ++end;
    }
    unsigned n = 0;
    if (end > i + 1 && (end == type.size() || !is_ident(type[end])) &&
        !type.slice(i + 1, end).getAsInteger(10, n)) {
      used.insert(n);
    }
    i = end - 1;
  }
}

bool isBoundArgument(const TranslationRule::ExprRule &rule, unsigned n) {
  return llvm::any_of(rule.generics, [&](const auto &bounds) {
    return llvm::any_of(bounds, [&](const TranslationRule::Bound &bound) {
      return llvm::is_contained(bound.args, n);
    });
  });
}

unsigned iteratorOverGeneric(const TranslationRule::ExprRule &rule,
                             unsigned n) {
  for (unsigned i = 0; i < rule.generics.size(); ++i) {
    for (const auto &bound : rule.generics[i]) {
      if (llvm::StringRef(bound.trait_name).ends_with("Iterator") &&
          llvm::is_contained(bound.args, n)) {
        return i + 1;
      }
    }
  }
  return 0;
}

void checkGenerics(const std::string &dir, const std::string &name,
                   const clang::FunctionDecl *decl,
                   const TranslationRule::ExprRule &rule) {
  std::unordered_set<std::string> declared;
  if (auto tmpl = decl->getDescribedFunctionTemplate()) {
    for (auto param : *tmpl->getTemplateParameters()) {
      declared.insert(param->getName().str());
    }
  }
  std::unordered_set<unsigned> used;
  collectGenerics(rule.body, used);
  collectGenerics(rule.return_type.type, used);
  for (const auto &param : rule.params) {
    collectGenerics(param.type, used);
  }
  collectGenerics(rule.init.type, used);
  for (unsigned i : used) {
    if (!declared.contains(std::format("T{}", i)) &&
        !isBoundArgument(rule, i)) {
      llvm::errs() << dir << "::" << name << ": generic T" << i
                   << " used by the target but missing from src\n";
      llvm::report_fatal_error("Absent generic from src");
    }
  }
  if (rule.usesInit() &&
      (decl->param_empty() || !decl->parameters()
                                   .back()
                                   ->getType()
                                   ->getAs<clang::PackExpansionType>())) {
    llvm::errs() << dir << "::" << name << '\n';
    llvm::report_fatal_error(
        "Expr rule uses init but its src does not end in a pack");
  }
}

void addRule(clang::ASTContext &ctx, const std::string &dir,
             const std::string &name, const clang::NamedDecl *decl) {
  auto rule = findRule(Model::kUnsafe, dir, name);
  if (model_ == Model::kRefCount) {
    if (auto refcount_rule = findRule(model_, dir, name)) {
      rule = refcount_rule;
    }
  }
  assert(rule && "rule has no target");
  if (!name.starts_with('f')) {
    auto keys = RuleKeys::GetTypeKeys(*sema_, decl);
    assert(!keys.empty() && "type rule has no key");
    for (auto key : keys) {
      types_.emplace(
          key, TypeRuleDecl{decl, TranslationRule::ParseTypeRuleJSON(*rule)});
    }
    return;
  }
  auto expr_rule = TranslationRule::ParseExprRuleJSON(*rule);
  if (auto function = decl->getAsFunction()) {
    checkGenerics(dir, name, function, expr_rule);
  } else {
    assert(llvm::isa<clang::VarDecl>(decl) &&
           "expression rule is neither a function nor a variable");
  }
  auto keys = RuleKeys::GetExprKeys(*sema_, decl);
  assert(!keys.empty() && "expression rule has no key");
  for (const auto &key : keys) {
    exprs_.emplace(
        key, ExprRuleDecl{decl, TranslationRule::ParseExprRuleJSON(*rule)});
  }
}

void addCxxRules(clang::ASTContext &ctx, const clang::NamespaceDecl *rules) {
  for (auto child : rules->decls()) {
    auto ns = llvm::dyn_cast<clang::NamespaceDecl>(child);
    if (!ns) {
      continue;
    }
    auto dir = ns->getName();
    bool is_rule_dir = dir.consume_front("cpp2rust_rules_");
    if (!is_rule_dir) {
      llvm::report_fatal_error("rule namespace is not named after a rule dir");
    }
    for (auto decl : ns->decls()) {
      auto named = llvm::cast<clang::NamedDecl>(decl);
      if (llvm::isa<clang::TypeAliasTemplateDecl>(named) &&
          named->getName().starts_with(kInitAliasPrefix)) {
        continue;
      }
      if (!isRuleName(named->getName())) {
        llvm::report_fatal_error("declaration is not a rule");
      }
      addRule(ctx, dir.str(), named->getName().str(), named);
    }
  }
}

void addCRule(clang::ASTContext &ctx, const clang::NamedDecl *decl) {
  auto identifier = decl->getName();
  if (!identifier.consume_front("cpp2rust_rules_")) {
    return;
  }
  auto [dir, name] = identifier.rsplit('_');
  if (!isRuleName(name)) {
    llvm::report_fatal_error("declaration is not a rule");
  }
  addRule(ctx, dir.str(), name.str(), decl);
}

struct ExprMatch {
  ExprRuleDecl *rule_decl = nullptr;
  llvm::ArrayRef<clang::TemplateArgument> args;
};

ExprMatch findExpr(const clang::Expr *use) {
  ExprMatch best;
  auto consider = [&](const RuleKeys::ExprKey &key) {
    auto [begin, end] = exprs_.equal_range(key);
    for (auto it = begin; it != end; ++it) {
      auto args = AstMatcher::Match(*sema_, use, it->second.decl);
      if (!args) {
        continue;
      }
      if (best.rule_decl) {
        auto better = AstMatcher::MoreSpecialized(*sema_, use, it->second.decl,
                                                  best.rule_decl->decl);
        if (!better) {
          use->dump(llvm::errs(), sema_->Context);
          it->second.decl->dump(llvm::errs());
          best.rule_decl->decl->dump(llvm::errs());
          llvm::report_fatal_error(
              "use matches several rules, none more specialized");
        }
        if (better != it->second.decl) {
          continue;
        }
      }
      best = {&it->second, *args};
    }
  };
  auto key = RuleKeys::GetExprKey(sema_->Context, use);
  if (!key) {
    return best;
  }
  consider(*key);
  if (auto member = std::get_if<RuleKeys::Member>(&*key)) {
    consider(RuleKeys::Unqualified{member->name});
  }
  if (std::holds_alternative<RuleKeys::Constructor>(*key)) {
    auto type = RuleDecls::SkipImplicit(use)->getType();
    if (auto record = type.getCanonicalType()->getAsCXXRecordDecl();
        record && !type->isDependentType() && !IsUserDefinedDecl(record)) {
      consider(RuleKeys::kAnyConstructor);
    }
  }
  return best;
}

struct TypeRuleMatch {
  TypeRuleDecl *rule_decl = nullptr;
  AstMatcher::TypeMatch match;
};

TypeRuleMatch findType(clang::QualType use) {
  TypeRuleMatch best;
  auto &ctx = sema_->Context;
  auto consider = [&](clang::QualType type, bool templates) {
    for (auto key : RuleKeys::GetTypeKeys(ctx, type)) {
      auto [begin, end] = types_.equal_range(key);
      for (auto it = begin; it != end; ++it) {
        if (!templates && llvm::isa<clang::TemplateDecl>(it->second.decl)) {
          continue;
        }
        if (best.rule_decl && best.rule_decl->decl == it->second.decl) {
          continue;
        }
        auto match = AstMatcher::MatchType(*sema_, type, it->second.decl);
        if (!match) {
          continue;
        }
        if (best.rule_decl) {
          auto better = AstMatcher::MoreSpecializedType(*sema_, it->second.decl,
                                                        best.rule_decl->decl);
          if (!better) {
            type.dump(llvm::errs(), ctx);
            it->second.decl->dump(llvm::errs());
            best.rule_decl->decl->dump(llvm::errs());
            llvm::report_fatal_error(
                "type matches several rules, none more specialized");
          }
          if (better != it->second.decl) {
            continue;
          }
        }
        best = {&it->second, *std::move(match)};
      }
    }
  };
  auto skipped = RuleDecls::SkipInternalTypedefs(ctx, use);
  if (RuleDecls::SpelledTypedef(ctx, skipped) !=
      RuleDecls::SpelledTypedef(ctx, use)) {
    consider(use, /*templates=*/false);
    if (best.rule_decl) {
      return best;
    }
  }
  use = skipped;
  if (auto tag = use->getAsTagDecl();
      tag &&
      (IsUserDefinedDecl(tag) || tag->hasAttr<clang::TransparentUnionAttr>())) {
    return best;
  }
  consider(use, /*templates=*/true);
  if (!best.rule_decl) {
    auto spelled = RuleDecls::SpelledTypedef(ctx, use);
    if (spelled && !IsUserDefinedDecl(spelled)) {
      return findType(use.getCanonicalType());
    }
  }
  return best;
}

Matcher::Bindings getBindings(const clang::NamedDecl *rule,
                              llvm::ArrayRef<clang::TemplateArgument> args) {
  Matcher::Bindings bindings;
  auto tmpl = llvm::dyn_cast<clang::TemplateDecl>(rule);
  if (!tmpl) {
    return bindings;
  }
  auto params = tmpl->getTemplateParameters();
  for (unsigned i = 0; i < params->size() && i < args.size(); ++i) {
    auto name = params->getParam(i)->getName();
    unsigned n = 0;
    if (!name.consume_front("T") || name.getAsInteger(10, n) ||
        args[i].getKind() == clang::TemplateArgument::Pack) {
      continue;
    }
    bindings.resize(std::max<size_t>(bindings.size(), n));
    bindings[n - 1] = args[i];
  }
  return bindings;
}

} // namespace

void Load(clang::Sema &sema, Model model) {
  auto &ctx = sema.Context;
  sema_ = &sema;
  model_ = model;
  exprs_.clear();
  types_.clear();
  for (auto decl : ctx.getTranslationUnitDecl()->decls()) {
    auto named = llvm::dyn_cast<clang::NamedDecl>(decl);
    if (!named || !named->getIdentifier()) {
      continue;
    }
    if (!ctx.getLangOpts().CPlusPlus) {
      addCRule(ctx, named);
    } else if (auto ns = llvm::dyn_cast<clang::NamespaceDecl>(named);
               ns && ns->getName() == "cpp2rust_rules") {
      addCxxRules(ctx, ns);
    }
  }
}

Matcher::Match<TranslationRule::ExprRule> Search(clang::ASTContext &ctx,
                                                 const clang::Expr *expr) {
  if (ExprHasUserDefinedDecl(expr)) {
    return {};
  }
  auto match = findExpr(expr);
  if (!match.rule_decl) {
    return {};
  }
  return {&match.rule_decl->rule,
          getBindings(match.rule_decl->decl, match.args)};
}

clang::QualType iteratorElementType(clang::ASTContext &ctx,
                                    clang::QualType iterator) {
  iterator = iterator.getNonReferenceType().getUnqualifiedType();
  if (iterator->isPointerType()) {
    return iterator->getPointeeType().getUnqualifiedType();
  }
  auto [rule, bindings] = Search(ctx, iterator);
  assert(rule && !bindings.empty() &&
         bindings[0].getKind() == clang::TemplateArgument::Type &&
         "iterator type rule declares no element generic T1");
  return bindings[0].getAsType();
}

clang::QualType GetGenericType(clang::ASTContext &ctx, const clang::Expr *expr,
                               unsigned n) {
  auto [rule, bindings] = Search(ctx, expr);
  assert(rule && "expression must have a translation rule");
  if (n <= bindings.size() &&
      bindings[n - 1].getKind() == clang::TemplateArgument::Type) {
    return bindings[n - 1].getAsType();
  }
  auto iterator = iteratorOverGeneric(*rule, n);
  assert(iterator &&
         "generic is neither bound by the src nor the element of an iterator");
  return iteratorElementType(ctx, GetGenericType(ctx, expr, iterator));
}

clang::QualType GetConversionSourceType(clang::ASTContext &ctx,
                                        const clang::Expr *expr, unsigned to) {
  auto rule = Search(ctx, expr).first;
  assert(rule && "expression must have a translation rule");
  for (const auto &bound : rule->generics.at(to - 1)) {
    if (bound.trait_name == "CppFrom" || bound.trait_name == "CppFromUnsafe") {
      assert(bound.args.size() == 1 && "CppFrom takes one generic");
      return GetGenericType(ctx, expr, bound.args[0]);
    }
  }
  llvm::report_fatal_error("converted generic has no CppFrom bound");
}

clang::QualType GetInitType(clang::ASTContext &ctx, const clang::Expr *expr) {
  auto match = findExpr(expr);
  assert(match.rule_decl && "expression must have a translation rule");
  const auto &rule = match.rule_decl->rule;
  if (rule.init_cpp_type.empty()) {
    llvm::StringRef type = rule.init.type;
    unsigned n = 0;
    [[maybe_unused]] bool bad =
        !type.consume_front("T") || type.getAsInteger(10, n);
    assert(!bad && "init type must be a bare generic or a cpp_type!");
    return GetGenericType(ctx, expr, n);
  }
  auto function = match.rule_decl->decl->getAsFunction();
  auto tmpl = function->getDescribedFunctionTemplate();
  assert(tmpl && "cpp_type! init in a non-template rule");
  clang::TypeAliasTemplateDecl *alias = nullptr;
  for (auto decl : function->getDeclContext()->lookup(
           &ctx.Idents.get(InitAliasName(model_, function->getName().str())))) {
    if ((alias = llvm::dyn_cast<clang::TypeAliasTemplateDecl>(decl))) {
      break;
    }
  }
  assert(alias && "cpp_type! init has no alias next to its rule");
  clang::MultiLevelTemplateArgumentList args(tmpl, match.args,
                                             /*Final=*/false);
  clang::Sema::InstantiatingTemplate inst(
      *sema_, expr->getBeginLoc(), const_cast<clang::FunctionDecl *>(function));
  auto type =
      sema_->SubstType(alias->getTemplatedDecl()->getUnderlyingType(), args,
                       expr->getBeginLoc(), clang::DeclarationName());
  assert(!type.isNull() && "cpp_type! init does not substitute");
  return type;
}

std::string InitCppType(Model model, const std::string &dir,
                        const std::string &name) {
  auto rule = findRule(model, dir, name);
  if (!rule) {
    rule = findRule(Model::kUnsafe, dir, name);
  }
  if (!rule) {
    return {};
  }
  auto cpp_type = rule->getString("init_cpp_type");
  return cpp_type ? cpp_type->str() : std::string();
}

std::string InitAliasName(Model model, const std::string &name) {
  return std::format("{}{}_{}", kInitAliasPrefix.str(),
                     model == Model::kRefCount ? "refcount" : "unsafe", name);
}

clang::QualType GetDeclaredReturnType(const clang::Expr *expr) {
  auto match = findExpr(expr);
  assert(match.rule_decl && "expression must have a translation rule");
  auto function = match.rule_decl->decl->getAsFunction();
  assert(function && "rule is not a function");
  auto tmpl = function->getDescribedFunctionTemplate();
  if (!tmpl) {
    return function->getReturnType();
  }
  clang::MultiLevelTemplateArgumentList args(tmpl, match.args,
                                             /*Final=*/false);
  clang::Sema::InstantiatingTemplate inst(
      *sema_, expr->getBeginLoc(), const_cast<clang::FunctionDecl *>(function));
  return sema_->SubstType(function->getReturnType(), args, expr->getBeginLoc(),
                          clang::DeclarationName());
}

bool HasExprRule(const clang::FunctionDecl *decl) {
  auto key = RuleKeys::GetDeclKey(decl);
  if (!key) {
    return false;
  }
  auto member = std::get_if<RuleKeys::Member>(&*key);
  return exprs_.contains(*key) ||
         (member && exprs_.contains(RuleKeys::Unqualified{member->name}));
}

Matcher::Match<TranslationRule::TypeRule> Search(clang::ASTContext &ctx,
                                                 clang::QualType qual_type) {
  auto match = findType(qual_type);
  if (!match.rule_decl) {
    return {};
  }
  auto bindings = getBindings(match.rule_decl->decl, match.match.args);
  if (auto generic = match.rule_decl->rule.generic) {
    return Search(ctx, bindings.at(generic - 1).getAsType());
  }
  return {&match.rule_decl->rule, std::move(bindings)};
}

const TranslationRule::ExprRule *GetExprRule(clang::ASTContext &ctx,
                                             const clang::Expr *expr) {
  return Search(ctx, expr).first;
}

bool MapsToPointer(clang::ASTContext &ctx, clang::QualType qual_type) {
  auto rule = Search(ctx, qual_type).first;
  if (!rule) {
    return pointsToUserType(qual_type);
  }
  return rule->type_info.is_pointer();
}

bool MapsToRefcountPointer(clang::ASTContext &ctx, clang::QualType qual_type) {
  auto rule = Search(ctx, qual_type).first;
  if (!rule) {
    return model_ == Model::kRefCount && pointsToUserType(qual_type);
  }
  return rule->type_info.is_refcount_pointer;
}

bool ReturnsPointer(clang::ASTContext &ctx, const clang::Expr *expr) {
  auto rule = Search(ctx, expr).first;
  return rule && rule->return_type.is_pointer();
}

bool ReturnsReference(const clang::Expr *expr) {
  if (ExprHasUserDefinedDecl(expr)) {
    return false;
  }
  auto match = findExpr(expr);
  return match.rule_decl && match.rule_decl->decl->getAsFunction()
                                ->getReturnType()
                                ->isReferenceType();
}

bool ParamIsPointer(clang::ASTContext &ctx, const clang::Expr *expr,
                    unsigned index) {
  auto rule = Search(ctx, expr).first;
  assert(rule && "expression must have a translation rule");
  return rule->params.at(index).is_pointer();
}

clang::Expr *BuildParamConversion(clang::ASTContext &ctx,
                                  const clang::Expr *expr, unsigned index,
                                  clang::Expr *arg) {
  auto [rule, bindings] = Search(ctx, expr);
  assert(rule && "expression must have a translation rule");
  llvm::StringRef type = rule->params.at(index).type;
  unsigned n = 0;
  if (!type.consume_front("T") || type.empty() || type.getAsInteger(10, n)) {
    return nullptr;
  }
  auto &binding = bindings.at(n - 1);
  assert(binding.getKind() == clang::TemplateArgument::Type &&
         "parameter typed by a non-type generic");
  auto target = binding.getAsType();
  if ((arg->getType()->isArrayType() && target->isPointerType()) ||
      ctx.hasSameUnqualifiedType(arg->getType().getNonReferenceType(),
                                 target)) {
    return nullptr;
  }
  auto *init = BuildInitExpr(*sema_, target, arg, arg->getExprLoc());
  assert(init && "argument cannot be converted to the parameter type");
  return init;
}

bool IsLibcPassthrough(clang::ASTContext &ctx, const clang::Expr *expr) {
  const auto *tgt_ir = GetExprRule(ctx, expr);
  if (tgt_ir == nullptr || !tgt_ir->body.empty() || !tgt_ir->is_extern) {
    return false;
  }
  if (const auto *call = clang::dyn_cast<clang::CallExpr>(expr)) {
    expr = call->getCallee();
  }
  const auto *ref =
      clang::dyn_cast<clang::DeclRefExpr>(expr->IgnoreParenImpCasts());
  const auto *decl = ref != nullptr ? ref->getDecl() : nullptr;
  return decl != nullptr &&
         decl->getASTContext().getSourceManager().isInSystemHeader(
             decl->getLocation());
}

Model CurrentModel() { return model_; }

} // namespace cpp2rust::RuleRegistry
