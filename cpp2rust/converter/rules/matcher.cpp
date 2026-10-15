// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/matcher.h"

#include <cassert>
#include <cctype>

#include "converter/converter.h"

namespace cpp2rust::Matcher {

std::string MapBinding(Converter &converter, const Bindings &bindings,
                       unsigned n) {
  return converter.GetTemplateArgumentAsString(bindings.at(n));
}

// Substitutes template arguments into a target template string. Each template
// parameter in `tgt_template` is replaced with its corresponding argument from
// `bindings`, which is either a type or a value.
//
// Example:
//   bindings     = { int, 4 }
//   tgt_template = "[T1; T2]"
//   result       = "[i32; 4_usize]"
std::string InstantiateTgt(Converter &converter, const Bindings &bindings,
                           const std::string &tgt_template) {
  assert(bindings.size() <= TranslationRule::kMaxGenerics &&
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
        MapBinding(converter, bindings, instantiated_template[pos + 1] - '1');
    instantiated_template.replace(pos, 2, repl);
    pos += repl.length();
  }
  return instantiated_template;
}

} // namespace cpp2rust::Matcher
