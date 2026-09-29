#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <optional>
#include <string>
#include <vector>

namespace cpp2rust::Matching {
// Concrete C++ types bound to the rule's T1, T2, ... placeholders.
using Bindings = std::vector<std::optional<std::string>>;

std::optional<Bindings> MatchTemplate(const std::string &template_str,
                                      const std::string &instantiated);
} // namespace cpp2rust::Matching
