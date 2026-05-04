// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <llvm/Support/CommandLine.h>

#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <vector>

#include "cpp2rust_lib.h"
#include "logging.h"
#include "rules_dir.h"

namespace fs = std::filesystem;

namespace {
llvm::cl::OptionCategory cpp2rust_cmdargs("Cpp2Rust options");

llvm::cl::opt<bool> Verbose("verbose", llvm::cl::desc("Enable verbose logging"),
                            llvm::cl::init(false),
                            llvm::cl::cat(cpp2rust_cmdargs));

llvm::cl::opt<std::string> CcFile("file",
                                  llvm::cl::desc("Path to the C++ file"),
                                  llvm::cl::value_desc("file.cpp"),
                                  llvm::cl::cat(cpp2rust_cmdargs));

llvm::cl::opt<std::string>
    BuildDir("dir",
             llvm::cl::desc("Directory that contains compile_commands.json"),
             llvm::cl::value_desc("dir"), llvm::cl::cat(cpp2rust_cmdargs));

llvm::cl::opt<std::string> RsFile("o", llvm::cl::desc("Path to the Rust file"),
                                  llvm::cl::value_desc("output.rs"),
                                  llvm::cl::Required,
                                  llvm::cl::cat(cpp2rust_cmdargs));

llvm::cl::opt<std::string>
    Model("model",
          llvm::cl::desc(
              "Name of the translation model (unsafe, refcount [default])"),
          llvm::cl::value_desc("model"), llvm::cl::init("refcount"),
          llvm::cl::cat(cpp2rust_cmdargs));

llvm::cl::opt<std::string>
    RulesDir("rules",
             llvm::cl::desc("Directory where translation rules are located"),
             llvm::cl::value_desc("rules"), llvm::cl::cat(cpp2rust_cmdargs));

llvm::cl::list<std::string> CXXFlags("cxxflags",
                                     llvm::cl::desc("Additional CXXFLAGS"),
                                     llvm::cl::value_desc("cxxflags"),
                                     llvm::cl::ZeroOrMore,
                                     llvm::cl::cat(cpp2rust_cmdargs));

} // namespace

int main(int argc, char *argv[]) {
  llvm::cl::HideUnrelatedOptions(cpp2rust_cmdargs);
  llvm::cl::ParseCommandLineOptions(argc, argv);

  cpp2rust::SetVerbose(Verbose);

  if (CcFile.empty() && BuildDir.empty()) {
    llvm::errs() << "ERROR: please provide either --file or --dir\n";
    return EXIT_FAILURE;
  }

  if (!CcFile.empty() && !BuildDir.empty()) {
    llvm::errs() << "ERROR: please provide only one of --file or --dir\n";
    return EXIT_FAILURE;
  }

  if (!BuildDir.empty() && !CXXFlags.empty()) {
    llvm::errs() << "ERROR: can't combine --dir with --cxxflags\n";
    return EXIT_FAILURE;
  }

  auto model = cpp2rust::Model::kRefCount;
  if (Model == "refcount") {
    // ok
  } else if (Model == "unsafe") {
    model = cpp2rust::Model::kUnsafe;
  } else {
    llvm::errs() << "ERROR: unknown model: " << Model << "\n";
    return EXIT_FAILURE;
  }

  std::string cc_code;
  if (!CcFile.empty()) {
    std::ifstream file(CcFile);
    if (!file) {
      llvm::errs() << "ERROR: failed to open " << CcFile << "\n";
      return EXIT_FAILURE;
    }
    cc_code = {std::istreambuf_iterator<char>(file),
               std::istreambuf_iterator<char>()};
    if (cc_code.empty()) {
      llvm::errs() << "ERROR: empty source file\n";
      return EXIT_FAILURE;
    }
  }

  std::vector<std::string_view> cxx_flags(CXXFlags.begin(), CXXFlags.end());

  if (RulesDir.empty() && !cpp2rust::ResolveRulesDir(RulesDir)) {
    return EXIT_FAILURE;
  }

  auto rs_code =
      BuildDir.empty()
          ? cpp2rust::TranspileSrc(cc_code, model, cxx_flags, RulesDir, CcFile)
          : cpp2rust::TranspileDir(BuildDir, model, RulesDir);

  if (rs_code.empty()) {
    llvm::errs() << "ERROR: empty output file\n";
    return EXIT_FAILURE;
  }

  std::ofstream file(RsFile);
  if (!file) {
    llvm::errs() << "ERROR: failed to open " << RsFile << "\n";
    return EXIT_FAILURE;
  }

  file << rs_code;
  file.close();

  // call rustfmt.
  std::string rustfmt_command = "rustfmt " + RsFile;
  if (std::system(rustfmt_command.c_str()) != 0) {
    llvm::errs() << "ERROR: failed to run rustfmt\n";
    return EXIT_FAILURE;
  }

  return EXIT_SUCCESS;
}
