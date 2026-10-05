// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTConsumer.h>
#include <clang/Frontend/CompilerInstance.h>
#include <clang/Frontend/FrontendAction.h>
#include <clang/Lex/Preprocessor.h>
#include <clang/Tooling/Tooling.h>
#include <llvm/ADT/SmallVector.h>
#include <llvm/ADT/StringRef.h>
#include <llvm/Support/CommandLine.h>
#include <llvm/Support/JSON.h>
#include <llvm/Support/MemoryBuffer.h>
#include <llvm/Support/raw_ostream.h>

#include <cstdlib>
#include <filesystem>
#include <memory>
#include <string>

#include "compat/platform_flags.h"
#include "converter/rules/rules_loader.h"
#include "rule_preprocessing/rule_index.h"

namespace fs = std::filesystem;

namespace cpp2rust {

class IndexAction : public clang::ASTFrontendAction {
public:
  explicit IndexAction(RuleFile &file) : file_(file) {}

  std::unique_ptr<clang::ASTConsumer>
  CreateASTConsumer(clang::CompilerInstance &CI, llvm::StringRef) override {
    class Consumer : public clang::ASTConsumer {
    public:
      Consumer(clang::CompilerInstance &CI, RuleFile &file)
          : CI_(CI), file_(file) {}

      void HandleTranslationUnit(clang::ASTContext &ctx) override {
        if (CI_.getDiagnostics().hasErrorOccurred()) {
          std::exit(EXIT_FAILURE);
        }
        CollectRuleFile(ctx, file_);
      }

    private:
      clang::CompilerInstance &CI_;
      RuleFile &file_;
    };
    return std::make_unique<Consumer>(CI, file_);
  }

  bool BeginSourceFileAction(clang::CompilerInstance &CI) override {
    CI.getPreprocessor().addPPCallbacks(
        MakeIncludeCollector(CI.getSourceManager(), file_));
    return true;
  }

private:
  RuleFile &file_;
};

void Parse(const fs::path &src_path, RuleFile &file,
           llvm::ArrayRef<llvm::StringRef> cxx_flags) {
  bool is_c = src_path.extension() == ".c";
  auto flags = getPlatformClangBeginFlags();
  flags.push_back("-isystem" + src_path.parent_path().string());
  if (!is_c) {
    flags.insert(flags.end(), cxx_flags.begin(), cxx_flags.end());
  }
  auto end_flags = getPlatformClangEndFlags();
  flags.insert(flags.end(), end_flags.begin(), end_flags.end());
  auto code = llvm::MemoryBuffer::getFile(src_path.string());
  if (!code) {
    llvm::errs() << "ERROR: cannot read " << src_path.string() << '\n';
    std::exit(EXIT_FAILURE);
  }
  clang::tooling::runToolOnCodeWithArgs(
      std::make_unique<IndexAction>(file), (*code)->getBuffer(), flags,
      src_path.string(), is_c ? CLANG_C_COMPILER : CLANG_CXX_COMPILER);
}

} // namespace cpp2rust

namespace {

llvm::cl::OptionCategory cat("cpp-rule-indexer options");

llvm::cl::opt<std::string>
    SrcDir("dir",
           llvm::cl::desc("Path to a rule directory containing src.c and/or "
                          "src.cpp."),
           llvm::cl::value_desc("rule-dir"), llvm::cl::Required,
           llvm::cl::cat(cat));

llvm::cl::opt<std::string>
    IrPath("ir",
           llvm::cl::desc("Path of the ir_src.json file of the rule "
                          "directory, written by cpp-rule-preprocessor."),
           llvm::cl::value_desc("ir_src.json"), llvm::cl::Required,
           llvm::cl::cat(cat));

llvm::cl::list<std::string> CXXFlags("cxxflags",
                                     llvm::cl::desc("Additional CXXFLAGS"),
                                     llvm::cl::value_desc("cxxflags"),
                                     llvm::cl::ZeroOrMore, llvm::cl::cat(cat));

} // namespace

int main(int argc, char *argv[]) {
  llvm::cl::HideUnrelatedOptions(cat);
  llvm::cl::ParseCommandLineOptions(argc, argv);

  auto ir = llvm::MemoryBuffer::getFile(IrPath.getValue());
  if (!ir) {
    llvm::errs() << "ERROR: cannot read " << IrPath.getValue() << '\n';
    return EXIT_FAILURE;
  }
  auto parsed = llvm::json::parse((*ir)->getBuffer());
  if (!parsed) {
    llvm::errs() << "ERROR: cannot parse " << IrPath.getValue() << ": "
                 << llvm::toString(parsed.takeError()) << '\n';
    return EXIT_FAILURE;
  }
  if (!parsed->getAsObject()) {
    llvm::errs() << "ERROR: " << IrPath.getValue() << " is not an object\n";
    return EXIT_FAILURE;
  }
  const auto &rules = *parsed->getAsObject();

  llvm::SmallVector<llvm::StringRef, 4> cxx_flags(CXXFlags.begin(),
                                                  CXXFlags.end());
  fs::path dir = SrcDir.getValue();
  auto ir_dir = fs::path(IrPath.getValue()).parent_path();
  for (const char *name : {"src.c", "src.cpp"}) {
    auto path = dir / name;
    if (!fs::exists(path)) {
      continue;
    }
    llvm::errs() << "Indexing " << path.string() << '\n';
    cpp2rust::RuleFile file;
    cpp2rust::Parse(path, file, cxx_flags);
    bool is_c = path.extension() == ".c";
    cpp2rust::WriteIndex(ir_dir.parent_path() /
                             cpp2rust::RulesLoader::kIndexDirName /
                             (is_c ? "c" : "cpp"),
                         ir_dir.filename().string(), is_c, rules, file);
  }
  return EXIT_SUCCESS;
}
