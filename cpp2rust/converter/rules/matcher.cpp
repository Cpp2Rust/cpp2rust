// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/matcher.h"

#include <cassert>
#include <cctype>

#include "converter/converter.h"

namespace cpp2rust::Matcher {

std::string MapBinding(Converter &converter, const Bindings &bindings,
                       unsigned n) {
  auto type = bindings.at(n);
  assert(!type.isNull() && "template parameter is not bound to a type");
  return converter.GetUnboxedTypeAsString(type);
}

// Substitutes concrete types into a target template string using the provided
// type mapping. Each template parameter in `tgt_template` is replaced with its
// corresponding instantiated type from `types`.
//
// Example:
//   types        = { int }
//   tgt_template = "Vec<T1>"
//   result       = "Vec<i32>"
std::string InstantiateTgt(Converter &converter, const Bindings &types,
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
    auto repl =
        MapBinding(converter, types, instantiated_template[pos + 1] - '1');
    instantiated_template.replace(pos, 2, repl);
    pos += repl.length();
  }
  return instantiated_template;
}

} // namespace cpp2rust::Matcher
