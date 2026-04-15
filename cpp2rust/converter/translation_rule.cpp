// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/translation_rule.h"

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/Expr.h>
#include <clang/AST/PrettyPrinter.h>
#include <clang/ASTMatchers/ASTMatchFinder.h>
#include <clang/ASTMatchers/ASTMatchers.h>
#include <clang/Frontend/FrontendActions.h>
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

using RuleMap = std::unordered_map<std::string, Rule>;

class Callback : public clang::ast_matchers::MatchFinder::MatchCallback {
public:
  explicit Callback(RuleMap &out) : out_(out) {}

  void run(const clang::ast_matchers::MatchFinder::MatchResult &R) override {
    if (auto var = R.Nodes.getNodeAs<clang::VarDecl>("tvar")) {
      Rule rule;
      rule.src = Mapper::ToString(var->getType());
      out_[var->getNameAsString()] = std::move(rule);
      return;
    }

    if (auto func = R.Nodes.getNodeAs<clang::FunctionDecl>("func")) {
      auto sym = func->getQualifiedNameAsString();
      auto add = [&](std::string src) {
        Rule rule;
        rule.src = std::move(src);
        out_[sym] = std::move(rule);
      };

      if (const auto *mcall =
              R.Nodes.getNodeAs<clang::CXXMemberCallExpr>("mcall")) {
        if (mcall->getDirectCallee()) {
          add(Mapper::ToString(mcall));
          return;
        }
      }
      if (const auto *fcall = R.Nodes.getNodeAs<clang::CallExpr>("fcall")) {
        if (fcall->getDirectCallee()) {
          add(Mapper::ToString(fcall));
          return;
        }
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
    }
  }

private:
  RuleMap &out_;
};

class ActionFactory : public clang::tooling::FrontendActionFactory {
public:
  explicit ActionFactory(RuleMap &out) : cb_(out) {
    using namespace clang::ast_matchers;
    finder_.addMatcher(
        returnStmt(
            isExpansionInMainFile(),
            hasReturnValue(ignoringImplicit(ignoringParenImpCasts(anyOf(
                cxxMemberCallExpr().bind("mcall"),
                callExpr(unless(cxxMemberCallExpr())).bind("fcall"),
                cxxConstructExpr().bind("ctor"),
                cxxFunctionalCastExpr(has(ignoringImplicit(
                    ignoringParenImpCasts(cxxConstructExpr().bind("ctor"))))),
                memberExpr(hasDeclaration(fieldDecl())).bind("muse"),
                unresolvedMemberExpr().bind("umuse"),
                declRefExpr(to(anyOf(enumConstantDecl().bind("enum_val"),
                                     decl(unless(parmVarDecl())).bind("decl"))))
                    .bind("declref"),
                unaryOperator(hasUnaryOperand(
                                  declRefExpr(to(decl(unless(parmVarDecl()))))))
                    .bind("udeclref"))))),
            hasAncestor(functionDecl(isDefinition(),
                                     matchesName("(^|::)f[0-9]+$"),
                                     isExpansionInMainFile())
                            .bind("func"))),
        &cb_);

    finder_.addMatcher(varDecl(hasGlobalStorage(), isExpansionInMainFile(),
                               matchesName("(^|::)t[0-9]+$"))
                           .bind("tvar"),
                       &cb_);
  }

  std::unique_ptr<clang::FrontendAction> create() override {
    class Wrapped : public clang::ASTFrontendAction {
      clang::ast_matchers::MatchFinder &F_;

    public:
      explicit Wrapped(clang::ast_matchers::MatchFinder &MF) : F_(MF) {}
      std::unique_ptr<clang::ASTConsumer>
      CreateASTConsumer(clang::CompilerInstance &, llvm::StringRef) override {
        return F_.newASTConsumer();
      }
    };
    return std::make_unique<Wrapped>(finder_);
  }

private:
  Callback cb_;
  clang::ast_matchers::MatchFinder finder_;
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
      std::get<ExprTgt>(rule.tgt).name = name;
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
  assert(!name.empty() && "ExprTgt name must not be empty");
  assert(!module.empty() && "ExprTgt module must not be empty");
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
    if (auto *expr_tgt = std::get_if<ExprTgt>(&rules.at(name).tgt)) {
      expr_tgt->module = dir.filename().string();
    }
  }

  std::vector<Rule> result;
  for (auto &[name, rule] : rules) {
    assert(!rule.src.empty() && "Rule loaded from IR but has no src");
    if (auto *expr_tgt = std::get_if<ExprTgt>(&rule.tgt)) {
      expr_tgt->validate(path.string() + ":" + name);
    }
    result.push_back(std::move(rule));
  }
  return result;
}

} // namespace cpp2rust::TranslationRule
