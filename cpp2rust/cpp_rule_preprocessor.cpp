// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <llvm/Support/CommandLine.h>
#include <llvm/Support/FormatVariadic.h>
#include <llvm/Support/JSON.h>
#include <llvm/Support/raw_ostream.h>

#include <cstdlib>
#include <filesystem>
#include <map>
#include <string>

#include "converter/rule_src_parser.h"
#include "rules_dir.h"

namespace fs = std::filesystem;

namespace {
llvm::cl::OptionCategory cat("cpp-rule-preprocessor options");

llvm::cl::opt<std::string>
    RulesDir("rules",
             llvm::cl::desc("Directory containing rule packages (each with a "
                            "src.cpp); ir_src.json is written next to each. "
                            "If omitted, auto-detected as in cpp2rust"),
             llvm::cl::value_desc("dir"), llvm::cl::cat(cat));
} // namespace

static void process(const fs::path &src) {
  llvm::errs() << "Preprocessing " << src.string() << '\n';
  auto strings = cpp2rust::RuleSrcParser::Extract(src);

  // Sort by name for deterministic output.
  std::map<std::string, std::string> sorted;
  for (auto &[k, v] : strings.functions) {
    sorted.emplace(k, v);
  }
  for (auto &[k, v] : strings.types) {
    sorted.emplace(k, v);
  }

  llvm::json::Object root;
  for (auto &[k, v] : sorted) {
    llvm::json::Object entry;
    entry["to_string"] = v;
    root.try_emplace(k, std::move(entry));
  }

  auto out_path = src.parent_path() / "ir_src.json";
  std::error_code ec;
  llvm::raw_fd_ostream out(out_path.string(), ec);
  if (ec) {
    llvm::errs() << "ERROR: failed to open " << out_path.string() << ": "
                 << ec.message() << '\n';
    std::exit(EXIT_FAILURE);
  }
  out << llvm::formatv("{0:2}", llvm::json::Value(std::move(root))) << '\n';
}

int main(int argc, char *argv[]) {
  llvm::cl::HideUnrelatedOptions(cat);
  llvm::cl::ParseCommandLineOptions(argc, argv);

  if (RulesDir.empty() && !cpp2rust::ResolveRulesDir(RulesDir)) {
    return EXIT_FAILURE;
  }

  for (const auto &entry :
       fs::recursive_directory_iterator(RulesDir.getValue())) {
    if (entry.is_regular_file() && entry.path().filename() == "src.cpp") {
      process(entry.path());
    }
  }
  return EXIT_SUCCESS;
}
