#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <filesystem>
#include <string>
#include <unordered_map>

namespace cpp2rust::RuleSrcParser {

struct SrcStrings {
  std::unordered_map<std::string, std::string> functions; // f<n> -> ToString
  std::unordered_map<std::string, std::string> types;     // t<n> -> ToString
};

SrcStrings Extract(const std::filesystem::path &src_path);

} // namespace cpp2rust::RuleSrcParser
