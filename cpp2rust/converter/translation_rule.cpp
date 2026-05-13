// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/translation_rule.h"

#include <llvm/Support/JSON.h>
#include <llvm/Support/MemoryBuffer.h>

#include <algorithm>
#include <string>
#include <vector>

#include "logging.h"

namespace cpp2rust::TranslationRule {

namespace {

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
  auto access = obj.getString("access");
  return {(unsigned)*obj.getInteger("arg"), ParseAccessJSON(*access)};
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
    if (auto str = frag_obj->getString("text")) {
      result.push_back(TextFragment{str->str()});
    } else if (auto n = frag_obj->getInteger("generic")) {
      result.push_back(GenericFragment{(unsigned)*n});
    } else if (auto *ph = frag_obj->getObject("placeholder")) {
      result.push_back(ParsePlaceholderFragmentJSON(*ph));
    } else if (auto *mc = frag_obj->getObject("method_call")) {
      result.push_back(std::make_unique<MethodCallFragment>(
          ParseMethodCallFragmentJSON(*mc)));
    }
  }
  return result;
}

ExprRule ParseExprRuleJSON(const llvm::json::Object &obj) {
  ExprRule ir;

  if (auto *params = obj.getObject("params")) {
    for (auto &[key, val] : *params) {
      if (auto *param_obj = val.getAsObject()) {
        size_t id = atoi(llvm::StringRef(key).data() + 1);
        ir.params.resize(std::max(ir.params.size(), id + 1));
        ir.params[id] = ParseTypeInfoJSON(*param_obj);
      }
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
        size_t id = atoi(llvm::StringRef(key).data() + 1) - 1; // starts in T1
        ir.generics.resize(std::max(ir.generics.size(), id + 1));
        ir.generics[id] = std::move(bounds);
      }
    }
  }

  if (auto *body = obj.getArray("body")) {
    ir.body = ParseBodyFragmentsJSON(*body);
  }

  return ir;
}

TypeRule ParseTypeRuleJSON(const llvm::json::Object &obj) {
  TypeRule rule;
  if (auto init = obj.getString("init"))
    rule.initializer = init->str();
  rule.type_info = ParseTypeInfoJSON(obj);
  return rule;
}

bool CfgMatchesHost(llvm::StringRef cfg) {
#if defined(__linux__)
  return cfg == "linux";
#elif defined(__APPLE__)
  return cfg == "macos";
#else
  return false;
#endif
}

void LoadTgtFromIR(ExprRules &exprs, TypeRules &types,
                   const std::filesystem::path &json_path) {
  auto buf = llvm::MemoryBuffer::getFile(json_path.string());
  if (!buf)
    return;

  auto parsed = llvm::json::parse((*buf)->getBuffer());
  if (!parsed) {
    llvm::errs() << "Failed to parse IR JSON: " << json_path << ": "
                 << llvm::toString(parsed.takeError()) << '\n';
    assert(0);
    return;
  }

  auto *root = parsed->getAsObject();
  if (!root)
    return;

  for (auto &[entry_name, entry_val] : *root) {
    auto *obj = entry_val.getAsObject();
    if (!obj)
      continue;

    if (auto cfg = obj->getString("cfg"); cfg && !CfgMatchesHost(*cfg)) {
      continue;
    }

    auto name = entry_name.str();
    if (name[0] == 'f') {
      exprs[std::move(name)] = ParseExprRuleJSON(*obj);
    } else if (name[0] == 't') {
      types[std::move(name)] = ParseTypeRuleJSON(*obj);
    }
  }
}

void LoadIrSrc(ExprRules &exprs, TypeRules &types,
               const std::filesystem::path &json_path) {
  auto buf = llvm::MemoryBuffer::getFile(json_path.string());
  if (!buf) {
    llvm::errs() << "Missing " << json_path << ", run cpp-rule-preprocessor\n";
    assert(0);
    return;
  }

  auto parsed = llvm::json::parse((*buf)->getBuffer());
  if (!parsed) {
    llvm::errs() << "Failed to parse IR src JSON: " << json_path << ": "
                 << llvm::toString(parsed.takeError()) << '\n';
    assert(0);
    return;
  }

  auto *root = parsed->getAsObject();
  if (!root) {
    return;
  }

  for (auto &[entry_name, entry_val] : *root) {
    auto name = entry_name.str();
    auto val = entry_val.getAsString();
    if (name[0] == 'f') {
      auto it = exprs.find(name);
      if (it == exprs.end()) {
        llvm::errs() << name << '\n';
        assert(0 && "ir_src.json expr entry has no matching IR target rule");
      }
      it->second.src = val->str();
    } else if (name[0] == 't') {
      auto it = types.find(name);
      if (it == types.end()) {
        llvm::errs() << name << '\n';
        assert(0 && "ir_src.json type entry has no matching IR target rule");
      }
      it->second.src = val->str();
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

void TextFragment::dump() const { log() << "  text: \"" << text << "\"\n"; }

void PlaceholderFragment::dump() const {
  log() << "  placeholder: " << n;
  switch (access) {
  case Access::kRead:
    log() << " (read)\n";
    break;
  case Access::kWrite:
    log() << " (write)\n";
    break;
  case Access::kMove:
    log() << " (move)\n";
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
  log() << "  method_call:\n"
           "    receiver:\n";
  for (const auto &frag : receiver) {
    BodyFragmentDump(frag);
  }
  log() << "    body:\n";
  for (const auto &frag : body) {
    BodyFragmentDump(frag);
  }
}

void ExprRule::dump() const {
  log() << "Matching: " << src << '\n';
  unsigned i = 0;
  for (auto &info : params) {
    log() << "  param a" << i++ << ": ";
    info.dump();
    log() << '\n';
  }
  if (!return_type.type.empty()) {
    log() << "  return: ";
    return_type.dump();
    log() << '\n';
  }
  i = 0;
  for (auto &bounds : generics) {
    log() << "  generic T" << ++i << ':';
    for (auto &b : bounds) {
      log() << ' ' << b;
    }
    log() << '\n';
  }
  for (const auto &frag : body) {
    BodyFragmentDump(frag);
  }
}

void GenericFragment::dump() const { log() << "  generic: " << n << '\n'; }

void TypeInfo::dump() const {
  log() << type;
  if (is_refcount_pointer)
    log() << " [rc_ptr]";
  if (is_unsafe_pointer)
    log() << " [unsafe_ptr]";
}

void TypeRule::dump() const {
  log() << "name: " << src << "\n  Rust type: ";
  type_info.dump();
  log() << '\n';
  if (!initializer.empty()) {
    log() << "  init: " << initializer << '\n';
  }
}

std::pair<ExprRules, TypeRules> Load(const std::filesystem::path &path,
                                     Model model) {
  ExprRules exprs;
  TypeRules types;
  auto dir = path.parent_path();
  LoadTgtFromIR(exprs, types, dir / "ir_unsafe.json");

  if (model == Model::kRefCount) {
    auto refcount_ir_path = dir / "ir_refcount.json";
    if (std::filesystem::exists(refcount_ir_path)) {
      LoadTgtFromIR(exprs, types, refcount_ir_path);
    }
  }

  LoadIrSrc(exprs, types, dir / "ir_src.json");

  for (auto &[name, rule] : exprs) {
    if (rule.src.empty()) {
      llvm::errs() << name << '\n';
      rule.dump();
      assert(0 && "Expr rule loaded from IR but has no src");
    }
  }
  for (auto &[name, rule] : types) {
    if (rule.src.empty()) {
      llvm::errs() << name << '\n';
      rule.dump();
      assert(0 && "Type rule loaded from IR but has no src");
    }
  }
  return {std::move(exprs), std::move(types)};
}

} // namespace cpp2rust::TranslationRule
