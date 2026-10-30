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

ConvertFragment ParseConvertFragmentJSON(const llvm::json::Object &obj) {
  ConvertFragment cv;
  cv.to = obj.getInteger("to").value_or(0);
  cv.is_move = obj.getBoolean("move").value_or(false);
  if (auto *src = obj.getArray("src")) {
    cv.src = ParseBodyFragmentsJSON(*src);
  }
  return cv;
}

RefExprFragment ParseRefExprFragmentJSON(const llvm::json::Object &obj) {
  RefExprFragment re;
  re.prefix = obj.getString("prefix").value_or("").str();
  if (auto *inner = obj.getArray("inner")) {
    re.inner = ParseBodyFragmentsJSON(*inner);
  }
  return re;
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
    } else if (auto n = frag_obj->getInteger("default")) {
      result.push_back(DefaultFragment{(unsigned)*n});
    } else if (auto *ph = frag_obj->getObject("placeholder")) {
      result.push_back(ParsePlaceholderFragmentJSON(*ph));
    } else if (auto *mc = frag_obj->getObject("method_call")) {
      result.push_back(std::make_unique<MethodCallFragment>(
          ParseMethodCallFragmentJSON(*mc)));
    } else if (auto *re = frag_obj->getObject("ref_expr")) {
      result.push_back(
          std::make_unique<RefExprFragment>(ParseRefExprFragmentJSON(*re)));
    } else if (auto *cv = frag_obj->getObject("convert")) {
      result.push_back(
          std::make_unique<ConvertFragment>(ParseConvertFragmentJSON(*cv)));
    } else if (frag_obj->get("va_args")) {
      result.push_back(VaArgsFragment{});
    } else if (frag_obj->get("init")) {
      result.push_back(InitFragment{});
    }
  }
  return result;
}

void BodyFragmentDump(const BodyFragment &frag) {
  std::visit(
      [](const auto &f) {
        if constexpr (requires { f->dump(); }) {
          f->dump();
        } else {
          f.dump();
        }
      },
      frag);
}

} // namespace

bool AnyFragment(const std::vector<BodyFragment> &body,
                 llvm::function_ref<bool(const BodyFragment &)> pred) {
  return std::any_of(body.begin(), body.end(), [&](const BodyFragment &frag) {
    if (pred(frag)) {
      return true;
    }
    if (auto *mc = std::get_if<std::unique_ptr<MethodCallFragment>>(&frag)) {
      return AnyFragment((*mc)->receiver, pred) ||
             AnyFragment((*mc)->body, pred);
    }
    if (auto *re = std::get_if<std::unique_ptr<RefExprFragment>>(&frag)) {
      return AnyFragment((*re)->inner, pred);
    }
    if (auto *cv = std::get_if<std::unique_ptr<ConvertFragment>>(&frag)) {
      return AnyFragment((*cv)->src, pred);
    }
    return false;
  });
}

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
      log() << ' ' << b.trait_name;
      for (unsigned arg : b.args) {
        log() << " T" << arg;
      }
    }
    log() << '\n';
  }
  for (const auto &frag : body) {
    BodyFragmentDump(frag);
  }
}

bool ExprRule::usesInit() const {
  return AnyFragment(body, [](const BodyFragment &frag) {
    return std::holds_alternative<InitFragment>(frag);
  });
}

const std::vector<BodyFragment> *ExprRule::refExprOperand() const {
  if (body.size() != 1) {
    return nullptr;
  }
  auto *re = std::get_if<std::unique_ptr<RefExprFragment>>(&body[0]);
  return re ? &(*re)->inner : nullptr;
}

void RefExprFragment::dump() const {
  log() << "  ref_expr: " << prefix << '\n';
  for (auto &frag : inner) {
    BodyFragmentDump(frag);
  }
}

void ConvertFragment::dump() const {
  log() << "  convert: -> T" << to << (is_move ? " move" : "") << '\n';
  for (auto &frag : src) {
    BodyFragmentDump(frag);
  }
}

void GenericFragment::dump() const { log() << "  generic: " << n << '\n'; }

void DefaultFragment::dump() const { log() << "  default: " << n << '\n'; }

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
    log() << "  init:\n";
    for (auto &frag : initializer) {
      BodyFragmentDump(frag);
    }
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
      auto *arr = val.getAsArray();
      if (!arr) {
        continue;
      }
      std::vector<Bound> bounds;
      for (auto &b : *arr) {
        auto *bound = b.getAsObject();
        if (!bound) {
          continue;
        }
        Bound parsed;
        parsed.trait_name = bound->getString("trait").value_or("").str();
        if (auto *args = bound->getArray("args")) {
          for (auto &arg : *args) {
            llvm::StringRef name = arg.getAsString().value_or("");
            unsigned n = 0;
            if (name.consume_front("T") && !name.empty() &&
                !name.getAsInteger(10, n)) {
              parsed.args.push_back(n);
            }
          }
        }
        bounds.push_back(std::move(parsed));
      }
      size_t id = atoi(llvm::StringRef(key).data() + 1) - 1; // starts in T1
      ir.generics.resize(std::max(ir.generics.size(), id + 1));
      ir.generics[id] = std::move(bounds);
    }
  }

  if (auto *body = obj.getArray("body")) {
    ir.body = ParseBodyFragmentsJSON(*body);
  }

  return ir;
}

TypeRule ParseTypeRuleJSON(const llvm::json::Object &obj) {
  TypeRule rule;
  if (auto *init = obj.getArray("init"))
    rule.initializer = ParseBodyFragmentsJSON(*init);
  if (auto generic = obj.getInteger("generic"))
    rule.generic = *generic;
  rule.type_info = ParseTypeInfoJSON(obj);
  return rule;
}

} // namespace cpp2rust::TranslationRule
