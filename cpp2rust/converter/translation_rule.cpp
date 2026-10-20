// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/translation_rule.h"

#include <llvm/Support/JSON.h>

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
  if (auto *arr = obj.getArray("derives")) {
    for (const auto &elem : *arr) {
      if (auto s = elem.getAsString())
        info.derives.emplace_back(s->str());
    }
  }
  return info;
}

Access ParseAccessJSON(llvm::StringRef value) {
  if (value == "borrow") {
    return Access::kBorrow;
  } else if (value == "borrow_mut") {
    return Access::kBorrowMut;
  } else if (value == "move") {
    return Access::kMove;
  } else if (value == "take") {
    return Access::kTake;
  } else {
    llvm::errs() << "Invalid access value: " << value << '\n';
    assert(0);
    return Access::kBorrow;
  }
}

PlaceholderFragment
ParsePlaceholderFragmentJSON(const llvm::json::Object &obj) {
  auto access = obj.getString("access");
  return {
      (unsigned)*obj.getInteger("arg"),
      ParseAccessJSON(*access),
      obj.getBoolean("is_index_base").value_or(false),
  };
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
    } else if (frag_obj->get("va_args")) {
      result.push_back(VaArgsFragment{});
    } else if (frag_obj->get("init")) {
      result.push_back(InitFragment{});
    }
  }
  return result;
}

void BodyFragmentDump(const BodyFragment &frag) {
  if (auto *t = std::get_if<TextFragment>(&frag)) {
    t->dump();
  } else if (auto *p = std::get_if<PlaceholderFragment>(&frag)) {
    p->dump();
  } else if (auto *g = std::get_if<GenericFragment>(&frag)) {
    g->dump();
  } else if (auto *v = std::get_if<VaArgsFragment>(&frag)) {
    v->dump();
  } else if (auto *i = std::get_if<InitFragment>(&frag)) {
    i->dump();
  } else if (auto *mc =
                 std::get_if<std::unique_ptr<MethodCallFragment>>(&frag)) {
    (*mc)->dump();
  }
}

bool HasInitFragment(const std::vector<BodyFragment> &body) {
  return std::any_of(body.begin(), body.end(), [](const BodyFragment &frag) {
    if (auto *mc = std::get_if<std::unique_ptr<MethodCallFragment>>(&frag)) {
      return HasInitFragment((*mc)->receiver) || HasInitFragment((*mc)->body);
    }
    return std::holds_alternative<InitFragment>(frag);
  });
}

} // namespace

void TextFragment::dump() const { log() << "  text: \"" << text << "\"\n"; }

void VaArgsFragment::dump() const { log() << "  va_args\n"; }

void InitFragment::dump() const { log() << "  init\n"; }

void PlaceholderFragment::dump() const {
  log() << "  placeholder: " << n;
  switch (access) {
  case Access::kBorrow:
    log() << " (borrow)\n";
    break;
  case Access::kBorrowMut:
    log() << " (borrow_mut)\n";
    break;
  case Access::kMove:
    log() << " (move)\n";
    break;
  case Access::kTake:
    log() << " (take)\n";
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

bool ExprRule::usesInit() const { return HasInitFragment(body); }

void GenericFragment::dump() const { log() << "  generic: " << n << '\n'; }

void TypeInfo::dump() const {
  log() << type;
  if (is_refcount_pointer)
    log() << " [rc_ptr]";
  if (is_unsafe_pointer)
    log() << " [unsafe_ptr]";
  for (const auto &d : derives)
    log() << " +" << d;
}

void TypeRule::dump() const {
  log() << "Rust type: ";
  type_info.dump();
  log() << '\n';
  if (!initializer.empty()) {
    log() << "  init: " << initializer << '\n';
  }
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

  if (auto *rt = obj.getObject("return_type")) {
    ir.return_type = ParseTypeInfoJSON(*rt);
  }

  if (auto ms = obj.getBoolean("multi_statement")) {
    ir.multi_statement = *ms;
  }

  if (auto v = obj.getBoolean("is_extern")) {
    ir.is_extern = *v;
  }

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
  if (auto generic = obj.getInteger("generic"))
    rule.generic = *generic;
  rule.type_info = ParseTypeInfoJSON(obj);
  return rule;
}

} // namespace cpp2rust::TranslationRule
