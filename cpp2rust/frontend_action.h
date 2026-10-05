#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTConsumer.h>
#include <clang/Frontend/CompilerInstance.h>
#include <clang/Frontend/FrontendAction.h>
#include <clang/Lex/PPCallbacks.h>
#include <clang/Lex/Preprocessor.h>
#include <clang/Lex/PreprocessorOptions.h>
#include <clang/Tooling/Tooling.h>
#include <llvm/Support/MemoryBuffer.h>

#include <memory>
#include <string>

#include "ast_consumer.h"
#include "converter/factory.h"
#include "rules_loader.h"

namespace cpp2rust {
class SilenceRulesDiagnostics : public clang::PPCallbacks {
public:
  SilenceRulesDiagnostics(clang::CompilerInstance &CI,
                          const RulesLoader::PragmaHandler &handler)
      : CI_(CI), handler_(handler) {}

  void LexedFileChanged(clang::FileID FID, LexedFileChangeReason Reason,
                        clang::SrcMgr::CharacteristicKind FileType,
                        clang::FileID PrevFID,
                        clang::SourceLocation Loc) override {
    if (Reason == LexedFileChangeReason::ExitFile && PrevFID.isValid() &&
        PrevFID == handler_.rules_file()) {
      CI_.getDiagnostics().setSuppressAllDiagnostics(false);
    }
  }

private:
  clang::CompilerInstance &CI_;
  const RulesLoader::PragmaHandler &handler_;
};

class FrontendAction : public clang::ASTFrontendAction {
public:
  explicit FrontendAction(std::string &rs_code, Model model, bool first,
                          const std::string &rules_dir)
      : rs_code_(rs_code), model_(model), first_(first), rules_dir_(rules_dir) {
  }

  std::unique_ptr<clang::ASTConsumer>
  CreateASTConsumer(clang::CompilerInstance &CI,
                    llvm::StringRef InFile) override {
    return std::make_unique<ASTConsumer>(rs_code_, model_, first_, CI,
                                         rules_dir_);
  }

  bool BeginInvocation(clang::CompilerInstance &CI) override {
    for (const auto &input : CI.getFrontendOpts().Inputs) {
      auto path = input.getFile();
      auto buffer = CI.getFileManager().getBufferForFile(path);
      if (!buffer) {
        continue;
      }
      auto code = (*buffer)->getBuffer().str() + "\n;\n#pragma " +
                  RulesLoader::kPragmaName + "\n";
      CI.getPreprocessorOpts().addRemappedFile(
          path, llvm::MemoryBuffer::getMemBufferCopy(code, path).release());
    }
    return true;
  }

  bool BeginSourceFileAction(clang::CompilerInstance &CI) override {
    auto *handler = new RulesLoader::PragmaHandler(CI, rules_dir_);
    CI.getPreprocessor().AddPragmaHandler(handler);
    CI.getPreprocessor().addPPCallbacks(
        std::make_unique<SilenceRulesDiagnostics>(CI, *handler));
    return true;
  }

private:
  std::string &rs_code_;
  Model model_;
  bool first_;
  const std::string &rules_dir_;
};

class FrontendActionFactory : public clang::tooling::FrontendActionFactory {
public:
  explicit FrontendActionFactory(std::string &rs_code, Model model,
                                 const std::string &rules_dir)
      : rs_code_(rs_code), model_(model), rules_dir_(rules_dir) {}

  std::unique_ptr<clang::FrontendAction> create() override {
    bool f = first_;
    first_ = false;
    return std::make_unique<FrontendAction>(rs_code_, model_, f, rules_dir_);
  }

private:
  std::string &rs_code_;
  Model model_;
  bool first_ = true;
  const std::string &rules_dir_;
};
} // namespace cpp2rust
