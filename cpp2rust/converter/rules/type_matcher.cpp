// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/type_matcher.h"

#include <clang/AST/DeclCXX.h>
#include <clang/AST/DeclTemplate.h>
#include <llvm/ADT/STLExtras.h>

#include <optional>
#include <vector>

#include "converter/rules/decls.h"

namespace cpp2rust::AstMatcher {

namespace {

std::optional<std::vector<clang::TemplateArgument>>
completeArgs(clang::Sema &sema, const clang::TypeAliasTemplateDecl *tmpl,
             std::vector<clang::TemplateArgument> deduced) {
  if (llvm::none_of(deduced, [](const clang::TemplateArgument &arg) {
        return arg.isNull();
      })) {
    return deduced;
  }
  auto loc = tmpl->getLocation();
  clang::TemplateArgumentListInfo explicit_args(loc, loc);
  for (const auto &arg : deduced) {
    if (arg.isNull()) {
      break;
    }
    explicit_args.addArgument(
        sema.getTrivialTemplateArgumentLoc(arg, clang::QualType(), loc));
  }
  RuleDecls::Trial trial(sema);
  clang::Sema::CheckTemplateArgumentInfo info;
  if (sema.CheckTemplateArgumentList(
          const_cast<clang::TypeAliasTemplateDecl *>(tmpl), loc, explicit_args,
          /*DefaultArgs=*/{}, /*PartialTemplateArgs=*/false, info) ||
      trial.failed()) {
    return std::nullopt;
  }
  return std::vector<clang::TemplateArgument>(info.CanonicalConverted.begin(),
                                              info.CanonicalConverted.end());
}

bool isSameSugar(const clang::Type *a, const clang::Type *b) {
  auto a_typedef = llvm::dyn_cast<clang::TypedefType>(a);
  auto b_typedef = llvm::dyn_cast<clang::TypedefType>(b);
  if (a_typedef || b_typedef) {
    return a_typedef && b_typedef &&
           a_typedef->getDecl()->getCanonicalDecl() ==
               b_typedef->getDecl()->getCanonicalDecl();
  }
  return a == b;
}

} // namespace

std::optional<TypeMatch> MatchType(clang::Sema &sema, clang::QualType use,
                                   const clang::NamedDecl *rule) {
  auto &ctx = sema.Context;
  auto spelled = RuleDecls::PeelSugar(ctx, use).getUnqualifiedType();
  auto tmpl = llvm::dyn_cast<clang::TypeAliasTemplateDecl>(rule);
  auto alias = tmpl ? tmpl->getTemplatedDecl()
                    : llvm::cast<clang::TypedefNameDecl>(rule);
  auto pattern = alias->getUnderlyingType();
  if (!tmpl) {
    if (auto sugar = RuleDecls::GetSugar(pattern)) {
      if (!isSameSugar(spelled.getTypePtr(), sugar)) {
        return std::nullopt;
      }
      return TypeMatch{};
    }
    if (RuleDecls::GetSugar(spelled) || !ctx.hasSameUnqualifiedType(pattern, spelled)) {
      return std::nullopt;
    }
    return TypeMatch{};
  }
  auto params = tmpl->getTemplateParameters();
  if (auto member = pattern->getAs<clang::DependentNameType>()) {
    auto qualifier = member->getQualifier().getAsType();
    auto typedef_type =
        llvm::dyn_cast<clang::TypedefType>(spelled.getTypePtr());
    if (!qualifier || !typedef_type ||
        typedef_type->getDecl()->getIdentifier() != member->getIdentifier()) {
      return std::nullopt;
    }
    auto owner = llvm::dyn_cast<clang::CXXRecordDecl>(
        typedef_type->getDecl()->getDeclContext());
    if (!owner) {
      return std::nullopt;
    }
    auto args =
        RuleDecls::DeduceFromType(sema, params, clang::QualType(qualifier, 0),
                                  ctx.getCanonicalTagType(owner));
    if (args && (args = completeArgs(sema, tmpl, *std::move(args)))) {
      return TypeMatch{*std::move(args)};
    }
    return std::nullopt;
  }
  if (RuleDecls::GetSugar(spelled) && !pattern->getAs<clang::TemplateTypeParmType>()) {
    return std::nullopt;
  }
  auto args = RuleDecls::DeduceFromType(sema, params, pattern, spelled);
  if (args && (args = completeArgs(sema, tmpl, *std::move(args)))) {
    return TypeMatch{*std::move(args)};
  }
  return std::nullopt;
}

const clang::NamedDecl *MoreSpecializedType(clang::Sema &sema,
                                            const clang::NamedDecl *a,
                                            const clang::NamedDecl *b) {
  auto a_tmpl = llvm::dyn_cast<clang::TemplateDecl>(a);
  auto b_tmpl = llvm::dyn_cast<clang::TemplateDecl>(b);
  if (!a_tmpl || !b_tmpl) {
    if (!a_tmpl && b_tmpl) {
      return a;
    }
    return a_tmpl && !b_tmpl ? b : nullptr;
  }
  auto a_alias = llvm::dyn_cast<clang::TypeAliasTemplateDecl>(a);
  auto b_alias = llvm::dyn_cast<clang::TypeAliasTemplateDecl>(b);
  if (a_alias && b_alias) {
    auto ordered = [](const clang::TypeAliasTemplateDecl *alias) {
      auto type = alias->getTemplatedDecl()->getUnderlyingType();
      if (auto member = type->getAs<clang::DependentNameType>()) {
        return clang::QualType(member->getQualifier().getAsType(), 0);
      }
      return type;
    };
    auto instance_of = [&](const clang::TypeAliasTemplateDecl *x,
                           const clang::TypeAliasTemplateDecl *y) {
      return RuleDecls::DeduceFromType(sema, y->getTemplateParameters(),
                                       ordered(y),
                                       ordered(x).getCanonicalType())
          .has_value();
    };
    bool a_of_b = instance_of(a_alias, b_alias);
    bool b_of_a = instance_of(b_alias, a_alias);
    if (a_of_b != b_of_a) {
      return a_of_b ? a : b;
    }
  }
  return nullptr;
}

} // namespace cpp2rust::AstMatcher
