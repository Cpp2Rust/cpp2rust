// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/matcher.h"

#include <cassert>
#include <cctype>

namespace cpp2rust::Matcher {

Bindings MapBindings(const Bindings &bindings) {
  Bindings mapped(bindings.size());
  for (unsigned i = 0; i < bindings.size(); ++i) {
    if (bindings[i]) {
      mapped[i] = MapBinding(bindings, i);
    }
  }
  return mapped;
}

// Substitutes concrete types into a target template string using the provided
// type mapping. Each template parameter in `tgt_template` is replaced with its
// corresponding instantiated type from `types`.
//
// Example:
//   types        = { {"i32"} }
//   tgt_template = "Vec<T1>"
//   result       = "Vec<i32>"
std::string InstantiateTgt(const Bindings &types,
                           const std::string &tgt_template) {
  assert(types.size() <= TranslationRule::kMaxGenerics &&
         "template placeholder exceeds kMaxGenerics");
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
    const auto &repl = types.at(instantiated_template[pos + 1] - '1').value();
    instantiated_template.replace(pos, 2, repl);
    pos += repl.length();
  }
  return instantiated_template;
}

} // namespace cpp2rust::Matcher
