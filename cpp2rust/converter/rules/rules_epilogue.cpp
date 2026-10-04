// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/rules_epilogue.h"

#include <algorithm>
#include <filesystem>
#include <format>
#include <vector>

namespace fs = std::filesystem;

namespace cpp2rust {

namespace {

void addCxxRules(std::string &out, const fs::path &rule_dir) {
  out += std::format("namespace cpp2rust_rules_{} {{\n",
                     rule_dir.filename().string());
  for (auto name : {"src.cpp", "src.c"}) {
    if (auto path = rule_dir / name; fs::exists(path)) {
      out += std::format("#include \"{}\"\n", path.string());
    }
  }
  out += "}\n";
}

void addCRules(std::string &out, const fs::path &rule_dir) {
  auto prefix = "cpp2rust_rules_" + rule_dir.filename().string();
  out += std::format("#define CPP2RUST_EXPR_RULE(n) {}_f##n\n", prefix);
  out += std::format("#define CPP2RUST_TYPE_RULE(n) {}_t##n\n", prefix);
  if (auto path = rule_dir / "src.c"; fs::exists(path)) {
    out += std::format("#include \"{}\"\n", path.string());
  }
  out += "#undef CPP2RUST_EXPR_RULE\n";
  out += "#undef CPP2RUST_TYPE_RULE\n";
}

} // namespace

RulesLanguage GetRulesLanguage(std::string_view filename) {
  return filename.ends_with(".c") ? RulesLanguage::kC : RulesLanguage::kCxx;
}

std::string GetRulesEpiloguePath(RulesLanguage language) {
  return std::string(RULES_SOURCE_DIR) + (language == RulesLanguage::kC
                                              ? "/cpp2rust_rules_epilogue.c"
                                              : "/cpp2rust_rules_epilogue.cpp");
}

bool IsRulesEpilogue(std::string_view path) {
  return path == GetRulesEpiloguePath(RulesLanguage::kC) ||
         path == GetRulesEpiloguePath(RulesLanguage::kCxx);
}

std::string BuildRulesEpilogue(RulesLanguage language) {
  std::vector<fs::path> rule_dirs;
  for (const auto &entry : fs::directory_iterator(RULES_SOURCE_DIR)) {
    if (entry.is_directory()) {
      rule_dirs.push_back(entry.path());
    }
  }
  std::sort(rule_dirs.begin(), rule_dirs.end());

  std::string out = "#pragma GCC system_header\n";
  if (language == RulesLanguage::kCxx) {
    out += "#define CPP2RUST_EXPR_RULE(n) f##n\n";
    out += "#define CPP2RUST_TYPE_RULE(n) t##n\n";
  }
  for (const auto &rule_dir : rule_dirs) {
    if (language == RulesLanguage::kCxx) {
      addCxxRules(out, rule_dir);
    } else {
      addCRules(out, rule_dir);
    }
  }
  return out;
}

} // namespace cpp2rust
