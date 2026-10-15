#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/Expr.h>
#include <clang/AST/TemplateBase.h>

#include <string>
#include <utility>
#include <vector>

#include "converter/translation_rule.h"

namespace cpp2rust {
class Converter;
}

namespace cpp2rust::Matcher {
using Bindings = std::vector<clang::TemplateArgument>;

template <typename Rule> using Match = std::pair<Rule *, Bindings>;

std::string MapBinding(Converter &converter, const Bindings &bindings,
                       unsigned n);

std::string InstantiateTgt(Converter &converter, const Bindings &bindings,
                           const std::string &tgt_template);
} // namespace cpp2rust::Matcher
