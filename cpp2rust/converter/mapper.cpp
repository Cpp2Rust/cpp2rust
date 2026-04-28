// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/mapper.h"

#include <clang/AST/ExprCXX.h>
#include <clang/Basic/SourceManager.h>
#include <llvm/Support/ThreadPool.h>

#include <atomic>
#include <format>
#include <mutex>
#include <regex>
#include <utility>
#include <vector>

#include "converter/converter_lib.h"
#include "converter/translation_rule.h"

namespace cpp2rust::Mapper {

namespace {

clang::ASTContext *ctx_ = nullptr;
Model model_ = Model::kUnsafe;
bool translation_rules_loaded_ = false;

std::unordered_map<std::string, TranslationRule::ExprTgt>
    exprs_; // src -> ExprTgt
std::unordered_map<std::string, TranslationRule::TypeTgt>
    types_; // src -> TypeTgt

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

// Attempts to unify an instantiated C++ type or function signature with a
// corresponding template pattern. If the two match structurally, it returns
// a mapping from template parameter names (e.g., "T1") to their concrete
// instantiated types (e.g., "int"). If no match is possible, returns nullopt.
//
// Example:
//   template_str   = "std::vector<T1>::vector()"
//   instantiated   = "std::vector<int>::vector()"
//   result         = { {"T1", "int"} }
std::optional<std::unordered_map<std::string, std::string>>
matchTemplate(const std::string &template_str,
              const std::string &instantiated) {
  auto matchLiteralAt = [&](const std::string &input_str, size_t pos,
                            const std::string &literal,
                            size_t &end_pos) -> bool {
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
                                      const std::string &lit) -> size_t {
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

  std::unordered_map<std::string, std::string> captured;

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

      std::string name = template_str.substr(ti, tj - ti);
      ti = tj;

      std::string nextLit;
      size_t scan = ti;
      while (scan < template_str.size()) {
        if (template_str[scan] == 'T' && scan + 1 < template_str.size() &&
            std::isdigit(template_str[scan + 1])) {
          break;
        }
        scan++;
      }
      nextLit = template_str.substr(ti, scan - ti);

      auto [it, inserted] = captured.try_emplace(std::move(name));
      if (!inserted) {
        size_t end_pos = 0;
        if (!matchLiteralAt(instantiated, si, it->second, end_pos)) {
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

          it->second = instantiated.substr(a, b - a);
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

          it->second = instantiated.substr(a, b - a);
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
        tj++;
      }

      std::string lit = template_str.substr(ti, tj - ti);
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
//   types        = { {"T1", "i32"} }
//   tgt_template = "Vec<T1>"
//   result       = "Vec<i32>"
std::string
instantiateTgt(const std::unordered_map<std::string, std::string> &types,
               const std::string &tgt_template) {
  std::string instantiated_template = tgt_template;
  for (const auto &[key, value] : types) {
    std::string::size_type pos = 0;
    while ((pos = instantiated_template.find(key, pos)) != std::string::npos) {
      instantiated_template.replace(pos, key.length(), value);
      pos += value.length();
    }
  }
  return instantiated_template;
}

template <class Map, class MatchPred>
Map::const_iterator parallel_search(const Map &container,
                                    MatchPred &&match_func) {
  if (container.empty()) {
    return container.cend();
  }

  auto tie_breaker = [](const std::string &a, const std::string &b) -> bool {
    if (a.size() != b.size()) {
      // Match more specific rules first (usually the longer ones).
      return a.size() > b.size();
    }
    return a < b; // Lexicographically
  };

  const unsigned hw = std::max(1u, std::thread::hardware_concurrency());
  const unsigned nthreads =
      std::min<unsigned>(hw, std::max<size_t>(1, container.bucket_count()));

  std::atomic<size_t> next_bucket{0};
  std::mutex hit_mtx;
  std::optional<typename Map::key_type> hit_key;

  auto worker = [&](unsigned) {
    while (true) {
      size_t b = next_bucket.fetch_add(1, std::memory_order_relaxed);
      if (b >= container.bucket_count()) {
        break;
      }

      for (auto it = container.cbegin(b); it != container.cend(b); ++it) {
        if (!match_func(it->first)) {
          continue;
        }

        std::scoped_lock lk(hit_mtx);
        if (!hit_key || tie_breaker(it->first, *hit_key)) {
          hit_key = it->first;
        }
      }
    }
  };

  {
    llvm::DefaultThreadPool pool(
        llvm::heavyweight_hardware_concurrency(nthreads));
    for (unsigned t = 0; t < nthreads; ++t)
      pool.async(worker, t);
    pool.wait();
  }

  return hit_key ? container.find(*hit_key) : container.cend();
}

decltype(exprs_)::const_iterator search(const clang::Expr *expr) {
  auto qualified_name = ToString(expr);
  auto result = parallel_search(exprs_, [&](const std::string &tpl) {
    return matchTemplate(tpl, qualified_name);
  });
  llvm::errs() << "search expr " << qualified_name << ", result:\n";
  if (result != exprs_.end()) {
    result->second.dump();
  } else {
    llvm::errs() << "None\n";
  }
  return result;
}

decltype(types_)::const_iterator search(clang::QualType qual_type) {
  auto type = ToString(qual_type);
  auto result = parallel_search(
      types_, [&](const std::string &tpl) { return matchTemplate(tpl, type); });
  llvm::errs() << "search type " << type << ", result: "
               << ((result == types_.end()) ? "None"
                                            : result->second.type_info.type)
               << '\n';
  return result;
}

void addRulesFromDirectory(const std::filesystem::path &dir, Model model) {
  for (const auto &entry : std::filesystem::recursive_directory_iterator(dir)) {
    auto &path = entry.path();
    if (entry.is_regular_file() && path.extension() == ".cpp") {
      auto rules = TranslationRule::Load(path, model);
      if (rules.empty()) {
        llvm::errs() << "No rules found in " << path << '\n';
        continue;
      }
      for (auto &rule : rules) {
        if (auto *expr = std::get_if<TranslationRule::ExprTgt>(&rule.tgt)) {
          if (!exprs_.try_emplace(std::move(rule.src), std::move(*expr))
                   .second) {
            llvm::errs() << "Key: " << rule.src << " already exists in exprs\n";
            assert(0);
          }
        } else if (auto *type =
                       std::get_if<TranslationRule::TypeTgt>(&rule.tgt)) {
          if (!types_.try_emplace(std::move(rule.src), std::move(*type))
                   .second) {
            llvm::errs() << "Key: " << rule.src << " already exists in types\n";
            assert(0);
          }
        }
      }
    }
  }
}

void addBuiltinTypes(Model model) {
  assert(ctx_);

  auto add_builtin_rule = [&](clang::QualType qt, const std::string &rust) {
    auto cxx = ToString(qt);
    types_[cxx] = TranslationRule::TypeTgt::Plain(rust);
    types_["const " + cxx] = TranslationRule::TypeTgt::Plain(rust);

    switch (model) {
    case Model::kUnsafe:
      types_[cxx + " *"] = TranslationRule::TypeTgt::UnsafePtr("*mut " + rust);
      types_["const " + cxx + " *"] =
          TranslationRule::TypeTgt::UnsafePtr("*const " + rust);
      break;
    case Model::kRefCount:
      types_[cxx + " *"] =
          TranslationRule::TypeTgt::RefcountPtr("Ptr::<" + rust + ">");
      types_["const " + cxx + " *"] =
          TranslationRule::TypeTgt::RefcountPtr("Ptr::<" + rust + ">");
      break;
    }
  };

  auto build_rust_type = [&](clang::QualType qt) {
    unsigned bits = ctx_->getTypeSize(qt);
    char sign = qt->isSignedIntegerType() ? 'i' : 'u';
    return std::string(1, sign) + std::to_string(bits);
  };

  // Misc
  add_builtin_rule(ctx_->BoolTy, "bool");
  add_builtin_rule(ctx_->FloatTy, "f32");
  add_builtin_rule(ctx_->DoubleTy, "f64");

  switch (model) {
  case Model::kUnsafe:
    types_[ToString(ctx_->VoidTy) + " *"] =
        TranslationRule::TypeTgt::UnsafePtr("*mut ::libc::c_void");
    types_["const " + ToString(ctx_->VoidTy) + " *"] =
        TranslationRule::TypeTgt::UnsafePtr("*const ::libc::c_void");
    break;
  case Model::kRefCount:
    types_[ToString(ctx_->VoidTy) + " *"] =
        TranslationRule::TypeTgt::RefcountPtr("AnyPtr");
    types_["const " + ToString(ctx_->VoidTy) + " *"] =
        TranslationRule::TypeTgt::RefcountPtr("AnyPtr");
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
  auto rule = parallel_search(types_, [&](const std::string &tpl) {
    return matchTemplate(tpl, cpp_type);
  });
  if (rule == types_.end()) {
    llvm::errs() << "cpp_type: " << cpp_type << '\n';
    assert(0 && "Type is not present in types_");
  }
  auto subs = matchTemplate(rule->first, cpp_type).value();
  for (auto &kv : subs) {
    kv.second = mapTypeStringRecursive(kv.second);
  }
  return instantiateTgt(subs, rule->second.type_info.type);
}

std::string normalizeTranslationRule(std::string rule) {
  const std::array<std::pair<std::regex, std::string>, 2> normalization_rules{{
      // Dettach pointer from double reference. Useful for matching translation
      // rules.
      {std::regex(R"(\*\&\&)"), "* &&"},
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
  return search(qual_type) != types_.end();
}

bool Contains(const clang::Expr *expr) { return search(expr) != exprs_.end(); }

const TranslationRule::ExprTgt *GetExprTgt(const clang::Expr *expr) {
  if (auto it = search(expr); it != exprs_.end()) {
    return &it->second;
  }
  return nullptr;
}

std::string MapFunctionName(const clang::FunctionDecl *decl) {
  assert(decl);
  if (exprs_.contains(ToString(decl))) {
    return std::format("libcc2rs::{}_{}", decl->getNameAsString(),
                       model_ == Model::kRefCount ? "refcount" : "unsafe");
  }
  return GetNamedDeclAsString(decl->getCanonicalDecl());
}

std::string InstantiateTemplate(const clang::Expr *expr,
                                const std::string &text) {
  auto it = search(expr);
  if (it == exprs_.end()) {
    return text;
  }
  auto types_map = matchTemplate(it->first, ToString(expr)).value();
  for (auto &kv : types_map) {
    kv.second = mapTypeStringRecursive(kv.second);
  }
  return instantiateTgt(types_map, text);
}

std::string Map(clang::QualType qual_type) {
  if (auto it = search(qual_type); it != types_.end()) {
    auto types_map = matchTemplate(it->first, ToString(qual_type)).value();
    for (auto &kv : types_map) {
      kv.second = mapTypeStringRecursive(kv.second);
    }
    return instantiateTgt(types_map, it->second.type_info.type);
  }
  return {};
}

bool MapsToPointer(clang::QualType qual_type) {
  if (auto it = search(qual_type); it != types_.end()) {
    return it->second.type_info.is_pointer();
  }
  return false;
}

bool MapsToRefcountPointer(clang::QualType qual_type) {
  if (auto it = search(qual_type); it != types_.end()) {
    return it->second.type_info.is_refcount_pointer;
  }
  return false;
}

bool ReturnsPointer(const clang::Expr *expr) {
  if (auto it = search(expr); it != exprs_.end()) {
    return it->second.return_type.is_pointer();
  }
  return false;
}

const TranslationRule::TypeInfo &GetParamInfo(const clang::Expr *expr,
                                              unsigned index) {
  auto name = "a" + std::to_string(index);
  auto it = search(expr);
  assert(it != exprs_.end() && "expression must have a translation rule");
  auto name_it = it->second.params.find(name);
  assert(name_it != it->second.params.end() &&
         "placeholder arg must have a corresponding param type in IR");
  return name_it->second;
}

std::string GetParamType(const clang::Expr *expr, unsigned index) {
  auto &info = GetParamInfo(expr, index);
  auto types_map = matchTemplate(search(expr)->first, ToString(expr)).value();
  for (auto &kv : types_map) {
    kv.second = mapTypeStringRecursive(kv.second);
  }
  return instantiateTgt(types_map, info.type);
}

bool ParamIsPointer(const clang::Expr *expr, unsigned index) {
  return GetParamInfo(expr, index).is_pointer();
}

void AddRuleForUserDefinedType(clang::NamedDecl *decl) {
  auto cpp_name = ToString(decl);
  auto rs_name = std::regex_replace(cpp_name, std::regex("::"), "_");

  if (!types_.try_emplace(cpp_name, TranslationRule::TypeTgt::Plain(rs_name))
           .second) {
    return;
  }

  if (auto record_decl = llvm::dyn_cast<clang::RecordDecl>(decl)) {
    // Forward declaration
    if (!record_decl->isThisDeclarationADefinition()) {
      return;
    }

    if (auto cxx_decl = llvm::dyn_cast<clang::CXXRecordDecl>(record_decl)) {
      if (cxx_decl->isAbstract()) {
        switch (model_) {
        case Model::kUnsafe:
          types_[cpp_name + " *"] =
              TranslationRule::TypeTgt::UnsafePtr("*mut dyn " + rs_name);
          break;
        case Model::kRefCount:
          types_[cpp_name + " *"] = TranslationRule::TypeTgt::RefcountPtr(
              "PtrDyn<dyn " + rs_name + '>');
          break;
        }
      } else {
        switch (model_) {
        case Model::kUnsafe:
          types_[cpp_name + " *"] =
              TranslationRule::TypeTgt::UnsafePtr("*mut " + rs_name);
          break;
        case Model::kRefCount:
          types_[cpp_name + " *"] =
              TranslationRule::TypeTgt::RefcountPtr("Ptr<" + rs_name + '>');
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

  std::string type;
  llvm::raw_string_ostream os(type);
  normalizeQualType(qual_type).print(os, getPrintPolicy());
  return normalizeTranslationRule(std::move(type));
}

std::string ToString(const clang::NamedDecl *decl) {
  if (auto *record = clang::dyn_cast<clang::RecordDecl>(decl);
      record && !record->getIdentifier()) {
    return synthesizeAnonRecordName(record);
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

  os << ToString(func_decl->getReturnType()) << " ";
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
    std::string opcode =
        clang::UnaryOperator::getOpcodeStr(uop->getOpcode()).str();
    return uop->isPostfix() ? ToString(uop->getSubExpr()) + std::move(opcode)
                            : std::move(opcode) + ToString(uop->getSubExpr());
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
  for (auto &[src, expr] : exprs_) {
    llvm::errs() << "Expr: " << src << '\n';
    expr.dump();
  }
  for (auto &[src, type_tgt] : types_) {
    llvm::errs() << "Type: " << src << '\n';
    type_tgt.dump();
  }
#endif
}

} // namespace cpp2rust::Mapper
