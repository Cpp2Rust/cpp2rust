#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <string>
#include <string_view>

namespace cpp2rust {

enum class RulesLanguage { kC, kCxx };

RulesLanguage GetRulesLanguage(std::string_view filename);

std::string GetRulesEpiloguePath(RulesLanguage language);

std::string BuildRulesEpilogue(RulesLanguage language);

bool IsRulesEpilogue(std::string_view path);

} // namespace cpp2rust
