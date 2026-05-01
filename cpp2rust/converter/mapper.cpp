// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/mapper.h"

#include <clang/AST/ExprCXX.h>
#include <clang/Basic/SourceManager.h>
#include <llvm/Support/ThreadPool.h>

#include <format>
#include <regex>
#include <unordered_map>
#include <utility>
#include <vector>

#include "converter/converter_lib.h"
#include "converter/translation_rule.h"

namespace cpp2rust::Mapper {

namespace {

clang::ASTContext *ctx_ = nullptr;
Model model_ = Model::kUnsafe;
bool translation_rules_loaded_ = false;

std::unordered_multimap<std::string, TranslationRule::ExprRule>
    exprs_; // src -> ExprRule
std::unordered_multimap<std::string, TranslationRule::TypeRule>
    types_; // src -> TypeRule

clang::PrintingPolicy getPrintPolicy() {
  assert(ctx_);
  clang::PrintingPolicy policy(ctx_->getLangOpts());
  policy.Bool = true;
  policy.SuppressTagKeyword = true;
  policy.SuppressScope = false;
  policy.FullyQualifiedName = true;
  policy.SuppressUnwrittenScope = true;
  return policy;
}

std::string GetExprMapKey(const std::string &str) {
  // Extract the function name from something like
  // const T1 & std::foo<T1, T2>::fn_name(args)
  auto n = str.find_first_of('(');
  if (n == std::string::npos) {
    n = str.size();
  }

  // Walk backwards from '(' tracking <> depth:
  // - skip characters inside template arguments (depth > 0)
  // - stop at the first space outside all angle brackets
  std::string result;
  int depth = 0;
  for (int i = (int)n - 1; i >= 0; --i) {
    char c = str[i];
    if (c == '>')
      ++depth;
    else if (c == '<')
      --depth;
    else if (c == ' ' && depth == 0)
      break;
    else if (depth == 0)
      result += c;
  }
  std::reverse(result.begin(), result.end());
  return result;
}

std::string GetTypeMapKey(const std::string &str) {
  auto n = str.find_first_of("<[");
  if (n == std::string::npos || str[n] == '<') {
    return str.substr(0, n);
  }
  // something like int[][] or T1[] -> []
  return str.substr(n + 1);
}

void AddTypeRule(std::string src, TranslationRule::TypeRule &&rule) {
  auto key = GetTypeMapKey(src);
  rule.src = std::move(src);
  types_.emplace(std::move(key), std::move(rule));
}

// Attempts to unify an instantiated C++ type or function signature with a
// corresponding template pattern. If the two match structurally, it returns
// a mapping from template parameter names (e.g., "T1") to their concrete
// instantiated types (e.g., "int"). If no match is possible, returns nullopt.
//
// Example:
//   template_str   = "std::vector<T1>::vector()"
//   instantiated   = "std::vector<int>::vector()"
//   result         = { "int" }
std::optional<std::vector<std::string>>
matchTemplate(const std::string &template_str,
              const std::string &instantiated) {
  auto matchLiteralAt = [&](const std::string &input_str, size_t pos,
                            std::string_view literal, size_t &end_pos) -> bool {
    size_t i = pos;
    size_t j = 0;

    while (true) {
      while (i < input_str.size() && std::isspace(input_str[i])) {
        i++;
      }

      while (j < literal.size() && std::isspace(literal[j])) {
        j++;
      }

      if (j == literal.size()) {
        end_pos = i;
        return true;
      }

      if (i >= input_str.size()) {
        return false;
      }

      if (input_str[i] != literal[j]) {
        return false;
      }

      i++;
      j++;
    }
  };

  auto findNextLiteralSameDepth = [&](const std::string &s, size_t start,
                                      std::string_view lit) -> size_t {
    int ang = 0;
    int par = 0;
    int sq = 0;

    for (size_t i = 0; i < s.size() && i < start; i++) {
      switch (s[i]) {
      case '<': {
        ang++;
        break;
      }
      case '>': {
        ang--;
        break;
      }
      case '(': {
        par++;
        break;
      }
      case ')': {
        par--;
        break;
      }
      case '[': {
        sq++;
        break;
      }
      case ']': {
        sq--;
        break;
      }
      default:
        break;
      }
      assert(ang >= 0 && par >= 0 && sq >= 0 && "Unbalanced ang, par or sq");
    }

    int base_ang = ang;
    int base_par = par;
    int base_sq = sq;

    for (size_t i = start; i <= s.size(); i++) {
      if (ang == base_ang && par == base_par && sq == base_sq) {
        size_t end_i = 0;
        if (matchLiteralAt(s, i, lit, end_i)) {
          return i;
        }
      }

      if (i == s.size()) {
        break;
      }

      char c = s[i];
      switch (c) {
      case '<': {
        ang++;
        break;
      }
      case '>': {
        ang--;
        break;
      }
      case '(': {
        par++;
        break;
      }
      case ')': {
        par--;
        break;
      }
      case '[': {
        sq++;
        break;
      }
      case ']': {
        sq--;
        break;
      }
      default:
        break;
      }

      if (ang < 0 || par < 0 || sq < 0) {
        return std::string::npos;
      }
    }

    return std::string::npos;
  };

  std::vector<std::string> captured;

  size_t ti = 0;
  size_t si = 0;

  while (ti < template_str.size()) {
    // Match T{N}. TODO: currently only T0..9 are supported
    if (template_str[ti] == 'T' && ti + 1 < template_str.size() &&
        std::isdigit(template_str[ti + 1])) {
      size_t tj = ti + 2;
      while (tj < template_str.size() && std::isdigit(template_str[tj])) {
        tj++;
      }

      size_t type_idx = std::stoi(&template_str[ti + 1]) - 1;
      ti = tj;

      std::string_view nextLit;
      size_t scan = ti;
      while (scan < template_str.size()) {
        if (template_str[scan] == 'T' && scan + 1 < template_str.size() &&
            std::isdigit(template_str[scan + 1])) {
          break;
        }
        scan++;
      }
      nextLit = std::string_view(template_str).substr(ti, scan - ti);

      captured.resize(std::max(captured.size(), type_idx + 1));
      auto &repl = captured[type_idx];
      if (!repl.empty()) {
        size_t end_pos = 0;
        if (!matchLiteralAt(instantiated, si, repl, end_pos)) {
          return std::nullopt;
        }
        si = end_pos;
      } else {
        if (!nextLit.empty()) {
          size_t k = findNextLiteralSameDepth(instantiated, si, nextLit);
          if (k == std::string::npos) {
            return std::nullopt;
          }

          size_t a = si;
          size_t b = k;

          while (a < b && std::isspace(instantiated[a])) {
            a++;
          }
          while (b > a && std::isspace(instantiated[b - 1])) {
            b--;
          }

          repl = instantiated.substr(a, b - a);
          si = k;
        } else {
          size_t a = si;
          size_t b = instantiated.size();

          while (a < b && std::isspace(instantiated[a])) {
            a++;
          }
          while (b > a && std::isspace(instantiated[b - 1])) {
            b--;
          }

          repl = instantiated.substr(a, b - a);
          si = instantiated.size();
        }
      }

      if (!nextLit.empty()) {
        size_t end_pos = 0;
        if (!matchLiteralAt(instantiated, si, nextLit, end_pos)) {
          return std::nullopt;
        }
        si = end_pos;
        ti += nextLit.size();
      }
    } else {
      size_t tj = ti;
      while (tj < template_str.size()) {
        if (template_str[tj] == 'T' && tj + 1 < template_str.size() &&
            std::isdigit(template_str[tj + 1])) {
          break;
        }
        ++tj;
      }

      auto lit = std::string_view(template_str).substr(ti, tj - ti);
      size_t end_pos = 0;
      if (!matchLiteralAt(instantiated, si, lit, end_pos)) {
        return std::nullopt;
      }
      si = end_pos;
      ti = tj;
    }
  }

  while (si < instantiated.size() && std::isspace(instantiated[si])) {
    si++;
  }

  if (si != instantiated.size()) {
    return std::nullopt;
  }

  return captured;
}

// Substitutes concrete types into a target template string using the provided
// type mapping. Each template parameter in `tgt_template` is replaced with its
// corresponding instantiated type from `types`.
//
// Example:
//   types        = { {"i32"} }
//   tgt_template = "Vec<T1>"
//   result       = "Vec<i32>"
std::string instantiateTgt(const std::vector<std::string> &types,
                           const std::string &tgt_template) {
  assert(types.size() <= 9);
  std::string instantiated_template = tgt_template;
  std::string::size_type pos = 0;
  while ((pos = instantiated_template.find('T', pos)) != std::string::npos) {
    if (pos + 1 >= instantiated_template.size()) {
      break;
    }
    if (!std::isdigit(instantiated_template[pos + 1])) {
      ++pos;
      continue;
    }
    auto &repl = types.at(instantiated_template[pos + 1] - '1');
    instantiated_template.replace(pos, 2, repl);
    pos += repl.length();
  }
  return instantiated_template;
}

template <typename T>
std::pair<T *, std::vector<std::string>>
search(std::unordered_multimap<std::string, T> &map, const std::string &txt,
       const std::string &key) {
  auto [it, end] = map.equal_range(key);
  T *rule = nullptr;
  std::vector<std::string> subs;

  for (; it != end; ++it) {
    auto &this_rule = it->second;
    auto this_subs = matchTemplate(this_rule.src, txt);
    if (!this_subs) {
      continue;
    }
    // tie breaker: prefer more specific rules (usually the longer ones)
    if (!rule || this_rule.src.size() > rule->src.size()) {
      rule = &this_rule;
      subs = *std::move(this_subs);
    }
  }
  return {rule, std::move(subs)};
}

TranslationRule::ExprRule *search(const clang::Expr *expr) {
  auto qualified_name = ToString(expr);
  auto [rule, subs] =
      search(exprs_, qualified_name, GetExprMapKey(qualified_name));
  llvm::errs() << "search expr " << qualified_name << ", result:\n";
  if (rule) {
    rule->dump();
  } else {
    llvm::errs() << "None\n";
  }
  return rule;
}

TranslationRule::TypeRule *search(clang::QualType qual_type) {
  auto type = ToString(qual_type);
  auto [rule, subs] = search(types_, type, GetTypeMapKey(type));
  llvm::errs() << "search type " << type
               << ", result: " << (rule ? rule->type_info.type : "None")
               << '\n';
  return rule;
}

void addRulesFromDirectory(const std::filesystem::path &dir, Model model) {
  for (const auto &entry : std::filesystem::recursive_directory_iterator(dir)) {
    auto &path = entry.path();
    if (entry.is_regular_file() && path.extension() == ".cpp") {
      auto [expr_rules, type_rules] = TranslationRule::Load(path, model);
      if (expr_rules.empty() && type_rules.empty()) {
        llvm::errs() << "No rules found in " << path << '\n';
        continue;
      }
      for (auto &[_, rule] : expr_rules) {
        exprs_.emplace(GetExprMapKey(rule.src), std::move(rule));
      }
      for (auto &[_, rule] : type_rules) {
        types_.emplace(GetTypeMapKey(rule.src), std::move(rule));
      }
    }
  }
}

void addBuiltinTypes(Model model) {
  assert(ctx_);

  auto add_builtin_rule = [&](clang::QualType qt, const std::string &rust) {
    auto cxx = ToString(qt);
    AddTypeRule(cxx, TranslationRule::TypeRule::Plain(rust));
    AddTypeRule("const " + cxx, TranslationRule::TypeRule::Plain(rust));

    switch (model) {
    case Model::kUnsafe:
      AddTypeRule(cxx + " *",
                  TranslationRule::TypeRule::UnsafePtr("*mut " + rust));
      AddTypeRule("const " + cxx + " *",
                  TranslationRule::TypeRule::UnsafePtr("*const " + rust));
      break;
    case Model::kRefCount:
      AddTypeRule(cxx + " *", TranslationRule::TypeRule::RefcountPtr(
                                  "Ptr::<" + rust + ">"));
      AddTypeRule("const " + cxx + " *", TranslationRule::TypeRule::RefcountPtr(
                                             "Ptr::<" + rust + ">"));
      break;
    }
  };

  auto build_rust_type = [&](clang::QualType qt) {
    unsigned bits = ctx_->getTypeSize(qt);
    char sign = qt->isSignedIntegerType() ? 'i' : 'u';
    return std::format("{}{}", sign, bits);
  };

  // Misc
  add_builtin_rule(ctx_->BoolTy, "bool");
  add_builtin_rule(ctx_->FloatTy, "f32");
  add_builtin_rule(ctx_->DoubleTy, "f64");

  switch (model) {
  case Model::kUnsafe:
    AddTypeRule(ToString(ctx_->VoidTy) + " *",
                TranslationRule::TypeRule::UnsafePtr("*mut ::libc::c_void"));
    AddTypeRule("const " + ToString(ctx_->VoidTy) + " *",
                TranslationRule::TypeRule::UnsafePtr("*const ::libc::c_void"));
    break;
  case Model::kRefCount:
    AddTypeRule(ToString(ctx_->VoidTy) + " *",
                TranslationRule::TypeRule::RefcountPtr("AnyPtr"));
    AddTypeRule("const " + ToString(ctx_->VoidTy) + " *",
                TranslationRule::TypeRule::RefcountPtr("AnyPtr"));
    break;
  }

  // Char
  add_builtin_rule(ctx_->CharTy, "u8");
  add_builtin_rule(ctx_->SignedCharTy, "i8");
  add_builtin_rule(ctx_->UnsignedCharTy, "u8");

  // Integers
  add_builtin_rule(ctx_->ShortTy, build_rust_type(ctx_->ShortTy));
  add_builtin_rule(ctx_->UnsignedShortTy,
                   build_rust_type(ctx_->UnsignedShortTy));
  add_builtin_rule(ctx_->IntTy, build_rust_type(ctx_->IntTy));
  add_builtin_rule(ctx_->UnsignedIntTy, build_rust_type(ctx_->UnsignedIntTy));
  add_builtin_rule(ctx_->LongTy, build_rust_type(ctx_->LongTy));
  add_builtin_rule(ctx_->UnsignedLongTy, build_rust_type(ctx_->UnsignedLongTy));
  add_builtin_rule(ctx_->LongLongTy, build_rust_type(ctx_->LongLongTy));
  add_builtin_rule(ctx_->UnsignedLongLongTy,
                   build_rust_type(ctx_->UnsignedLongLongTy));
}

clang::QualType normalizeQualType(clang::QualType qual_type) {
  assert(ctx_);

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
    qual_type = ctx_->getQualifiedType(qual_type, qualifiers);
  }

  if (isLRef) {
    qual_type = ctx_->getLValueReferenceType(qual_type);
  }

  if (isRRef) {
    qual_type = ctx_->getRValueReferenceType(qual_type);
  }

  return qual_type.getCanonicalType().getUnqualifiedType().getDesugaredType(
      *ctx_);
}

std::string mapTypeStringRecursive(const std::string &cpp_type) {
  auto [rule, subs] = search(types_, cpp_type, GetTypeMapKey(cpp_type));
  if (!rule) {
    llvm::errs() << "cpp_type: " << cpp_type << '\n';
    assert(0 && "Type is not present in types_");
  }
  for (auto &ty : subs) {
    ty = mapTypeStringRecursive(ty);
  }
  return instantiateTgt(subs, rule->type_info.type);
}

std::string normalizeTranslationRule(std::string rule) {
  // Detach pointer from double reference. Useful for matching translation
  // rules.
  rule = ReplaceAll(rule, "(*&&)", "* &&");

  const std::array<std::pair<std::regex, std::string>, 1> normalization_rules{{
      // Ignore constant template parameters, i.e. replace them with _.
      {std::regex(R"(\b\d+\b)"), "_"},
  }};

  for (const auto &r : normalization_rules) {
    rule = std::regex_replace(rule, r.first, r.second);
  }

  return rule;
}

static std::string synthesizeAnonRecordName(const clang::RecordDecl *record) {
  std::string parent_name;
  if (auto *parent =
          clang::dyn_cast<clang::RecordDecl>(record->getDeclContext())) {
    parent_name = parent->getIdentifier()
                      ? parent->getIdentifier()->getName().str()
                      : synthesizeAnonRecordName(parent);
    parent_name += '_';
  }
  return std::format("{}anon_{}", parent_name, GetAnonIndex(record));
}

} // namespace

PushASTContext::PushASTContext(clang::ASTContext &ctx) : prev_(ctx_) {
  ctx_ = &ctx;
}
PushASTContext::~PushASTContext() { ctx_ = prev_; }

bool Contains(clang::QualType qual_type) {
  return search(qual_type) != nullptr;
}

bool Contains(const clang::Expr *expr) { return search(expr) != nullptr; }

const TranslationRule::ExprRule *GetExprRule(const clang::Expr *expr) {
  return search(expr);
}

std::string MapFunctionName(const clang::FunctionDecl *decl) {
  assert(decl);
  if (exprs_.contains(GetExprMapKey(ToString(decl)))) {
    return std::format("libcc2rs::{}_{}", decl->getNameAsString(),
                       model_ == Model::kRefCount ? "refcount" : "unsafe");
  }
  return GetNamedDeclAsString(decl->getCanonicalDecl());
}

std::string InstantiateTemplate(const clang::Expr *expr, unsigned n) {
  auto expr_str = ToString(expr);
  auto [rule, subs] = search(exprs_, expr_str, GetExprMapKey(expr_str));
  auto text = std::format("T{}", n);
  if (!rule) {
    return text;
  }
  for (auto &ty : subs) {
    ty = mapTypeStringRecursive(ty);
  }
  return instantiateTgt(subs, text);
}

std::string Map(clang::QualType qual_type) {
  auto type_str = ToString(qual_type);
  auto [rule, subs] = search(types_, type_str, GetTypeMapKey(type_str));
  if (rule) {
    for (auto &ty : subs) {
      ty = mapTypeStringRecursive(ty);
    }
    return instantiateTgt(subs, rule->type_info.type);
  }
  return {};
}

bool MapsToPointer(clang::QualType qual_type) {
  auto rule = search(qual_type);
  return rule && rule->type_info.is_pointer();
}

bool MapsToRefcountPointer(clang::QualType qual_type) {
  auto rule = search(qual_type);
  return rule && rule->type_info.is_refcount_pointer;
}

bool ReturnsPointer(const clang::Expr *expr) {
  auto rule = search(expr);
  return rule && rule->return_type.is_pointer();
}

const TranslationRule::TypeInfo &GetParamInfo(const clang::Expr *expr,
                                              unsigned index) {
  auto rule = search(expr);
  assert(rule && "expression must have a translation rule");
  return rule->params.at(index);
}

std::string GetParamType(const clang::Expr *expr, unsigned index) {
  auto expr_str = ToString(expr);
  auto [rule, subs] = search(exprs_, expr_str, GetExprMapKey(expr_str));
  for (auto &ty : subs) {
    ty = mapTypeStringRecursive(ty);
  }
  return instantiateTgt(subs, rule->params.at(index).type);
}

bool ParamIsPointer(const clang::Expr *expr, unsigned index) {
  return GetParamInfo(expr, index).is_pointer();
}

void AddRuleForUserDefinedType(clang::NamedDecl *decl) {
  auto cpp_name = ToString(decl);
  auto rs_name = ReplaceAll(cpp_name, "::", "_");

  AddTypeRule(cpp_name, TranslationRule::TypeRule::Plain(rs_name));

  if (auto record_decl = llvm::dyn_cast<clang::RecordDecl>(decl)) {
    // Forward declaration
    if (!record_decl->isThisDeclarationADefinition()) {
      return;
    }

    if (auto cxx_decl = llvm::dyn_cast<clang::CXXRecordDecl>(record_decl)) {
      if (cxx_decl->isAbstract()) {
        switch (model_) {
        case Model::kUnsafe:
          AddTypeRule(cpp_name + " *", TranslationRule::TypeRule::UnsafePtr(
                                           "*mut dyn " + rs_name));
          break;
        case Model::kRefCount:
          AddTypeRule(cpp_name + " *", TranslationRule::TypeRule::RefcountPtr(
                                           "PtrDyn<dyn " + rs_name + '>'));
          break;
        }
      } else {
        switch (model_) {
        case Model::kUnsafe:
          AddTypeRule(cpp_name + " *",
                      TranslationRule::TypeRule::UnsafePtr("*mut " + rs_name));
          break;
        case Model::kRefCount:
          AddTypeRule(cpp_name + " *", TranslationRule::TypeRule::RefcountPtr(
                                           "Ptr<" + rs_name + '>'));
          break;
        }
      }

      for (auto *nested : GetNestedStructs(cxx_decl)) {
        AddRuleForUserDefinedType(nested);
      }
    }
  }
}

std::string ToString(clang::QualType qual_type) {
  assert(ctx_);

  if (auto cxx_record_decl = qual_type->getAsCXXRecordDecl()) {
    if (cxx_record_decl->isLambda()) {
      return ToString(cxx_record_decl->getLambdaCallOperator());
    }
  }

  if (auto *tag = qual_type->getAsTagDecl();
      tag && !tag->getIdentifier() && !tag->getTypedefNameForAnonDecl()) {
    return ToString(clang::cast<clang::NamedDecl>(tag));
  }

  std::string type;
  llvm::raw_string_ostream os(type);
  normalizeQualType(qual_type).print(os, getPrintPolicy());
  return normalizeTranslationRule(std::move(type));
}

std::string ToString(const clang::NamedDecl *decl) {
  if (auto *record = clang::dyn_cast<clang::RecordDecl>(decl);
      record && !record->getIdentifier()) {
    if (auto *typedef_decl = record->getTypedefNameForAnonDecl()) {
      return ToString(clang::cast<clang::NamedDecl>(typedef_decl));
    }
    return synthesizeAnonRecordName(record);
  }

  if (auto *enum_decl = clang::dyn_cast<clang::EnumDecl>(decl);
      enum_decl && !enum_decl->getIdentifier() &&
      !enum_decl->getTypedefNameForAnonDecl()) {
    return std::format("anon_enum_{}", GetLineNumber(enum_decl));
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
    decl->printQualifiedName(os, getPrintPolicy());
    return normalizeTranslationRule(std::move(out));
  }

  os << ToString(func_decl->getReturnType()) << ' ';
  if (const auto *method_decl =
          llvm::dyn_cast<clang::CXXMethodDecl>(func_decl)) {
    if (method_decl->getParent()->isLambda() &&
        method_decl->getOverloadedOperator() == clang::OO_Call) {
      func_decl->printName(os, getPrintPolicy());
    } else {
      func_decl->printQualifiedName(os, getPrintPolicy());
    }
  } else {
    func_decl->printQualifiedName(os, getPrintPolicy());
  }

  os << '(';
  for (unsigned i = 0, n = func_decl->getNumParams(); i < n; ++i) {
    if (i) {
      os << ", ";
    }
    os << ToString(func_decl->getParamDecl(i)->getType());
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

std::string ToString(const clang::Expr *expr) {
  if (!expr) {
    assert(0 && "!expr");
  }

  expr = expr->IgnoreParenImpCasts();

  if (const auto *CE = llvm::dyn_cast<clang::CallExpr>(expr)) {
    if (const auto *decl = CE->getDirectCallee()) {
      return ToString(decl);
    }
  }

  if (const auto *ctor = llvm::dyn_cast<clang::CXXConstructExpr>(expr)) {
    if (const auto *ctor_decl = ctor->getConstructor()) {
      return ToString(ctor_decl);
    }
    assert(0 && "expr is a CXXConstructExpr but could not get constructor");
  }

  if (const auto *ME = llvm::dyn_cast<clang::MemberExpr>(expr)) {
    if (const auto *member_decl =
            llvm::dyn_cast<clang::NamedDecl>(ME->getMemberDecl())) {
      if (const auto *method_decl =
              llvm::dyn_cast<clang::CXXMethodDecl>(member_decl)) {
        return ToString(method_decl);
      }
      if (ME->isArrow()) {
        auto *base = ME->getBase()->IgnoreParenImpCasts();
        if (auto *op = llvm::dyn_cast<clang::CXXOperatorCallExpr>(base)) {
          if (op->getOperator() == clang::OO_Arrow) {
            return ToString(op->getArg(0)->getType()) + "->" +
                   ToString(member_decl);
          }
        }
      } else if (auto for_range = GetParentForRange(*ctx_, ME)) {
        if (ToString(for_range->getRangeInit()->getType())
                .starts_with("std::map<")) {
          auto iter_type = GetForRangeIteratorType(for_range);
          if (!iter_type.isNull()) {
            return ToString(iter_type) + "->" + ToString(member_decl);
          }
        }
      }
      return ToString(member_decl);
    }
    assert(0 && "expr is a MemberExpr but could not get named decl");
  }

  if (const auto *decl_ref = llvm::dyn_cast<clang::DeclRefExpr>(expr)) {
    if (const auto *named_decl =
            llvm::dyn_cast<clang::NamedDecl>(decl_ref->getDecl())) {
      if (const auto *tmpl_decl =
              llvm::dyn_cast<clang::FunctionTemplateDecl>(named_decl)) {
        return ToString(tmpl_decl->getTemplatedDecl());
      }
      return ToString(named_decl);
    }
    return "";
  }

  if (const auto *uop = llvm::dyn_cast<clang::UnaryOperator>(expr)) {
    auto sub = ToString(uop->getSubExpr());
    std::string_view opcode =
        clang::UnaryOperator::getOpcodeStr(uop->getOpcode());
    return uop->isPostfix() ? std::format("{}{}", sub, opcode)
                            : std::format("{}{}", opcode, sub);
  }

  return "Unhandled case in ToString";
}

void LoadTranslationRules(Model model, clang::ASTContext &ctx,
                          const std::string &rules_dir) {
  ctx_ = &ctx;
  model_ = model;

  if (translation_rules_loaded_) {
    return;
  }
  translation_rules_loaded_ = true;

  addRulesFromDirectory(rules_dir, model);
  addBuiltinTypes(model);

#if 0
  for (auto &[src, rule] : exprs_) {
    llvm::errs() << "Expr key: " << src << '\n';
    rule.dump();
  }
  for (auto &[src, rule] : types_) {
    llvm::errs() << "Type key: " << src << '\n';
    rule.dump();
  }
#endif
}

} // namespace cpp2rust::Mapper
