// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/printer.h"

#include <clang/AST/ExprCXX.h>
#include <clang/Basic/OperatorKinds.h>
#include <clang/Basic/SourceManager.h>
#include <clang/Lex/Lexer.h>

#include <array>
#include <cctype>
#include <format>
#include <regex>
#include <string_view>
#include <utility>
#include <vector>

#include "converter/converter_lib.h"
#include "converter/mapper.h"

namespace cpp2rust::Printer {

namespace {

constexpr const char kPackMarker[] = "&&...";

clang::PrintingPolicy getPrintPolicy(clang::ASTContext &ctx) {
  clang::PrintingPolicy policy(ctx.getLangOpts());
  policy.Bool = true;
  policy.SuppressTagKeyword = true;
  policy.SuppressScope = false;
  policy.FullyQualifiedName = true;
  policy.SuppressUnwrittenScope = true;
  policy.UsePreferredNames = true;
  return policy;
}

clang::QualType normalizeQualType(clang::ASTContext &ctx,
                                  clang::QualType qual_type) {

  bool isLRef = qual_type->isLValueReferenceType();
  bool isRRef = qual_type->isRValueReferenceType();
  qual_type = qual_type.getNonReferenceType();

  clang::Qualifiers qualifiers = qual_type.getQualifiers();

  while (true) {
    if (const auto *attributed =
            llvm::dyn_cast<clang::AttributedType>(qual_type)) {
      qual_type = attributed->getModifiedType();
      continue;
    }
    if (const auto *dcltype = llvm::dyn_cast<clang::DecltypeType>(qual_type)) {
      qual_type = dcltype->getUnderlyingType();
      continue;
    }
    break;
  }

  if (llvm::isa<clang::InjectedClassNameType>(qual_type)) {
    qual_type = qual_type.getCanonicalType();
  }

  qual_type = qual_type.withFastQualifiers(qualifiers.getFastQualifiers());
  if (qualifiers.hasNonFastQualifiers()) {
    qual_type = ctx.getQualifiedType(qual_type, qualifiers);
  }

  if (isLRef) {
    qual_type = ctx.getLValueReferenceType(qual_type);
  }

  if (isRRef) {
    qual_type = ctx.getRValueReferenceType(qual_type);
  }

  return qual_type.getCanonicalType().getUnqualifiedType().getDesugaredType(
      ctx);
}

std::string normalizeTranslationRule(std::string rule) {
  // Detach pointer from double reference. Useful for matching translation
  // rules.
  ReplaceAll(rule, "*&&", "* &&");

  static const std::array<std::pair<std::regex, std::string>, 1>
      normalization_rules{{
          // Ignore constant template parameters, i.e. replace them with _.
          {std::regex(R"(\b\d+\b)"), "_"},
      }};

  for (const auto &r : normalization_rules) {
    rule = std::regex_replace(rule, r.first, r.second);
  }

  return rule;
}

} // namespace

std::string ToRustName(std::string name) {
  ReplaceAll(name, "::", "_");
  ReplaceAll(name, "*", "ptr");
  ReplaceAll(name, "&", "ref");
  ReplaceAll(name, "[", "arr");
  ReplaceAll(name, "]", "arr");
  ReplaceAll(name, "-", "neg");
  for (auto &c : name) {
    if (!std::isalnum(c) && c != '_') {
      c = '_';
    }
  }

  std::string_view stem(name);
  stem = stem.substr(0, stem.find_last_not_of('_') + 1);
  if (stem == "Ptr" || stem == "Value") {
    name += '_';
  }
  return name;
}

static std::string PointerToString(clang::ASTContext &ctx,
                                   clang::QualType pointee) {
  std::string out;
  llvm::raw_string_ostream os(out);
  ctx.getPointerType(pointee).print(os, getPrintPolicy(ctx));
  return normalizeTranslationRule(std::move(out));
}

std::vector<std::string> ToStringCandidates(clang::ASTContext &ctx,
                                            clang::QualType qual_type) {
  clang::QualType t = qual_type;
  if (const auto *decltype_type =
          clang::dyn_cast<clang::DecltypeType>(t.getTypePtr())) {
    t = decltype_type->getUnderlyingType();
  }
  if (const auto *typeof_type =
          clang::dyn_cast<clang::TypeOfExprType>(t.getTypePtr())) {
    t = typeof_type->getUnderlyingExpr()->getType();
  }
  if (!t->getAs<clang::TypedefType>() &&
      !t->getAs<clang::PredefinedSugarType>()) {
    if (const auto *ptr = t->getAs<clang::PointerType>()) {
      auto pointee = ptr->getPointeeType();
      auto canonical = pointee.getCanonicalType().getDesugaredType(ctx);
      bool builtin_alias = canonical->isBuiltinType() &&
                           (pointee->getAs<clang::TypedefType>() ||
                            pointee->getAs<clang::PredefinedSugarType>());
      if (!builtin_alias) {
        return {PointerToString(ctx, pointee), PointerToString(ctx, canonical)};
      }
    }
  }
  return {ToString(ctx, qual_type, ScalarSugar::kPreserve)};
}

std::string ToString(clang::ASTContext &ctx, clang::QualType qual_type,
                     ScalarSugar sugar) {

  if (sugar == ScalarSugar::kPreserve) {
    clang::QualType t = qual_type;
    if (const auto *decltype_type =
            clang::dyn_cast<clang::DecltypeType>(t.getTypePtr())) {
      t = decltype_type->getUnderlyingType();
    }
    if (const auto *typeof_type =
            clang::dyn_cast<clang::TypeOfExprType>(t.getTypePtr())) {
      t = typeof_type->getUnderlyingExpr()->getType();
    }
    if (const auto *typedef_type = t->getAs<clang::TypedefType>()) {
      if (t.getCanonicalType()->isBuiltinType()) {
        return typedef_type->getDecl()->getNameAsString();
      }
    } else if (const auto *predef = t->getAs<clang::PredefinedSugarType>()) {
      return predef->getIdentifier()->getName().str();
    } else if (const auto *ptr = t->getAs<clang::PointerType>()) {
      auto pointee = ptr->getPointeeType();
      auto canonical = pointee.getCanonicalType().getDesugaredType(ctx);
      bool builtin_alias = canonical->isBuiltinType() &&
                           (pointee->getAs<clang::TypedefType>() ||
                            pointee->getAs<clang::PredefinedSugarType>());
      if (builtin_alias) {
        return std::format("{}{}{} *",
                           pointee.isConstQualified() ? "const " : "",
                           pointee.isVolatileQualified() ? "volatile " : "",
                           ToString(ctx, pointee.getUnqualifiedType(),
                                    ScalarSugar::kPreserve));
      }
      if (Mapper::Map(ctx, pointee) == Mapper::Map(ctx, canonical)) {
        pointee = canonical;
      }
      return PointerToString(ctx, pointee);
    }
  }

  if (auto cxx_record_decl = qual_type->getAsCXXRecordDecl()) {
    if (cxx_record_decl->isLambda()) {
      return ToString(ctx, cxx_record_decl->getLambdaCallOperator());
    }
  }

  if (auto *tag = qual_type->getAsTagDecl();
      tag && !tag->getIdentifier() && !tag->getTypedefNameForAnonDecl()) {
    return ToString(ctx, clang::cast<clang::NamedDecl>(tag));
  }

  if (auto *tag = qual_type->getAsTagDecl();
      tag && tag->getIdentifier() &&
      tag->getDeclContext()->isFunctionOrMethod()) {
    return GetNamedDeclAsString(tag);
  }

  if (auto renamed = DisambiguateAnonymousTag(qual_type->getAsTagDecl());
      !renamed.empty()) {
    return renamed;
  }

  std::string type;
  llvm::raw_string_ostream os(type);
  normalizeQualType(ctx, qual_type).print(os, getPrintPolicy(ctx));
  return normalizeTranslationRule(std::move(type));
}

std::string ToString(clang::ASTContext &ctx, const clang::NamedDecl *decl) {
  if (auto *record = clang::dyn_cast<clang::RecordDecl>(decl);
      record && !record->getIdentifier()) {
    if (auto renamed = DisambiguateAnonymousTag(record); !renamed.empty()) {
      return renamed;
    }
    if (auto *typedef_decl = record->getTypedefNameForAnonDecl()) {
      return ToString(ctx, clang::cast<clang::NamedDecl>(typedef_decl));
    }
    return GetNamedDeclAsString(record);
  }

  if (auto *enum_decl = clang::dyn_cast<clang::EnumDecl>(decl)) {
    if (auto renamed = DisambiguateAnonymousTag(enum_decl); !renamed.empty()) {
      return renamed;
    }
    if (!enum_decl->getIdentifier() &&
        !enum_decl->getTypedefNameForAnonDecl()) {
      return GetNamedDeclAsString(enum_decl);
    }
  }

  std::string out;
  llvm::raw_string_ostream os(out);

  const clang::FunctionDecl *func_decl = nullptr;
  if (auto *template_decl = llvm::dyn_cast<clang::FunctionTemplateDecl>(decl)) {
    func_decl = template_decl->getTemplatedDecl();
  } else {
    func_decl = llvm::dyn_cast_or_null<clang::FunctionDecl>(decl);
  }

  if (!func_decl) {
    decl->printQualifiedName(os, getPrintPolicy(ctx));
    return normalizeTranslationRule(std::move(out));
  }

  os << ToString(ctx, func_decl->getReturnType()) << ' ';
  if (const auto op = func_decl->getOverloadedOperator();
      op >= clang::OverloadedOperatorKind::OO_LessLess &&
      op <= clang::OverloadedOperatorKind::OO_GreaterGreaterEqual) {
    // ensure matchTemplate does not consider these operator names when matching
    func_decl->getQualifier().print(os, getPrintPolicy(ctx));
    os << "operator ";
    switch (op) {
    case clang::OverloadedOperatorKind::OO_LessLess:
      os << "shl";
      break;
    case clang::OverloadedOperatorKind::OO_GreaterGreater:
      os << "shr";
      break;
    case clang::OverloadedOperatorKind::OO_LessLessEqual:
      os << "shleq";
      break;
    case clang::OverloadedOperatorKind::OO_GreaterGreaterEqual:
      os << "shreq";
      break;
    default:
      assert(0 && "Unexpected overloaded operator kind");
    }
  } else if (const auto *method_decl =
                 llvm::dyn_cast<clang::CXXMethodDecl>(func_decl)) {
    if (method_decl->getParent()->isLambda() &&
        method_decl->getOverloadedOperator() == clang::OO_Call) {
      func_decl->printName(os, getPrintPolicy(ctx));
    } else {
      func_decl->printQualifiedName(os, getPrintPolicy(ctx));
    }
  } else {
    func_decl->printQualifiedName(os, getPrintPolicy(ctx));
  }

  bool has_pack = HasFunctionParameterPack(func_decl);
  unsigned num_params = func_decl->getNumParams();
  if (has_pack) {
    const auto *primary = func_decl->getPrimaryTemplate();
    num_params =
        (primary ? primary->getTemplatedDecl() : func_decl)->getNumParams() - 1;
  }

  os << '(';
  for (unsigned i = 0; i < num_params; ++i) {
    if (i) {
      os << ", ";
    }
    os << ToString(ctx, func_decl->getParamDecl(i)->getType());
  }
  if (has_pack) {
    if (num_params) {
      os << ", ";
    }
    os << kPackMarker;
  }
  if (func_decl->isVariadic()) {
    if (func_decl->getNumParams()) {
      os << ", ";
    }
    os << "...";
  }
  os << ')';

  if (const auto *method_decl =
          llvm::dyn_cast<clang::CXXMethodDecl>(func_decl)) {
    if (method_decl->isConst()) {
      os << " const";
    }
    if (method_decl->isVolatile()) {
      os << " volatile";
    }
    switch (method_decl->getRefQualifier()) {
    case clang::RQ_LValue:
      os << " &";
      break;
    case clang::RQ_RValue:
      os << " &&";
      break;
    default:
      break;
    }
  }

  return normalizeTranslationRule(std::move(out));
}

std::string ToString(clang::ASTContext &ctx, const clang::Expr *expr) {
  if (!expr) {
    assert(0 && "!expr");
  }

  expr = expr->IgnoreParenImpCasts();

  if (llvm::isa<clang::IntegerLiteral>(expr) &&
      expr->getBeginLoc().isMacroID()) {
    auto &sm = ctx.getSourceManager();
    auto name = clang::Lexer::getImmediateMacroName(expr->getBeginLoc(), sm,
                                                    ctx.getLangOpts());
    if (!name.empty()) {
      return name.str();
    }
  }

  if (const auto *CE = llvm::dyn_cast<clang::CallExpr>(expr)) {
    if (const auto *decl = CE->getDirectCallee()) {
      return ToString(ctx, decl);
    }
  }

  if (const auto *ctor = llvm::dyn_cast<clang::CXXConstructExpr>(expr)) {
    if (const auto *ctor_decl = ctor->getConstructor()) {
      return ToString(ctx, ctor_decl);
    }
    assert(0 && "expr is a CXXConstructExpr but could not get constructor");
  }

  if (const auto *ME = llvm::dyn_cast<clang::MemberExpr>(expr)) {
    if (const auto *member_decl =
            llvm::dyn_cast<clang::NamedDecl>(ME->getMemberDecl())) {
      if (const auto *method_decl =
              llvm::dyn_cast<clang::CXXMethodDecl>(member_decl)) {
        return ToString(ctx, method_decl);
      }
      if (ME->isArrow()) {
        auto *base = ME->getBase()->IgnoreParenImpCasts();
        if (auto *op = llvm::dyn_cast<clang::CXXOperatorCallExpr>(base)) {
          if (op->getOperator() == clang::OO_Arrow) {
            return ToString(ctx, op->getArg(0)->getType()) + "->" +
                   ToString(ctx, member_decl);
          }
        }
      } else if (auto for_range = GetParentForRange(ctx, ME)) {
        if (ToString(ctx, for_range->getRangeInit()->getType())
                .starts_with("std::map<")) {
          auto iter_type = GetForRangeIteratorType(for_range);
          if (!iter_type.isNull()) {
            return ToString(ctx, iter_type) + "->" + ToString(ctx, member_decl);
          }
        }
      }
      return ToString(ctx, member_decl);
    }
    assert(0 && "expr is a MemberExpr but could not get named decl");
  }

  if (const auto *decl_ref = llvm::dyn_cast<clang::DeclRefExpr>(expr)) {
    if (const auto *named_decl =
            llvm::dyn_cast<clang::NamedDecl>(decl_ref->getDecl())) {
      if (const auto *tmpl_decl =
              llvm::dyn_cast<clang::FunctionTemplateDecl>(named_decl)) {
        return ToString(ctx, tmpl_decl->getTemplatedDecl());
      }
      return ToString(ctx, named_decl);
    }
    return "";
  }

  if (const auto *uop = llvm::dyn_cast<clang::UnaryOperator>(expr)) {
    auto sub = ToString(ctx, uop->getSubExpr());
    std::string_view opcode =
        clang::UnaryOperator::getOpcodeStr(uop->getOpcode());
    return uop->isPostfix() ? std::format("{}{}", sub, opcode)
                            : std::format("{}{}", opcode, sub);
  }

  return "Unhandled case in ToString";
}

} // namespace cpp2rust::Printer
