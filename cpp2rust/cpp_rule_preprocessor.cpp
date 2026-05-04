// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <llvm/Support/CommandLine.h>
#include <llvm/Support/FormatVariadic.h>
#include <llvm/Support/JSON.h>
#include <llvm/Support/raw_ostream.h>

#include <cstdlib>
#include <filesystem>
#include <string>

#include "converter/rule_src_parser.h"

namespace fs = std::filesystem;

namespace {
llvm::cl::OptionCategory cat("cpp-rule-preprocessor options");

llvm::cl::opt<std::string>
    SrcFile("file",
            llvm::cl::desc("Path to a rule's src.cpp. ir_src.json is written "
                           "next to it"),
            llvm::cl::value_desc("src.cpp"), llvm::cl::Required,
            llvm::cl::cat(cat));
} // namespace

int main(int argc, char *argv[]) {
  llvm::cl::HideUnrelatedOptions(cat);
  llvm::cl::ParseCommandLineOptions(argc, argv);

  fs::path src = SrcFile.getValue();
  llvm::errs() << "Preprocessing " << src.string() << '\n';
  llvm::json::Object root;
  cpp2rust::RuleSrcParser::Extract(src, root);

  auto out_path = src.parent_path() / "ir_src.json";
  std::error_code ec;
  llvm::raw_fd_ostream out(out_path.string(), ec);
  if (ec) {
    llvm::errs() << "ERROR: failed to open " << out_path.string() << ": "
                 << ec.message() << '\n';
    return EXIT_FAILURE;
  }
  out << llvm::formatv("{0:2}", llvm::json::Value(std::move(root))) << '\n';
  return EXIT_SUCCESS;
}
