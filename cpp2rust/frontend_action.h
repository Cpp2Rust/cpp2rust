#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTConsumer.h>
#include <clang/Frontend/CompilerInstance.h>
#include <clang/Frontend/FrontendAction.h>
#include <clang/Lex/PPCallbacks.h>
#include <clang/Lex/Preprocessor.h>
#include <clang/Tooling/Tooling.h>

#include <memory>
#include <string>

#include "ast_consumer.h"
#include "converter/factory.h"
#include "converter/rules/rules_epilogue.h"

namespace cpp2rust {
class SilenceRulesDiagnostics : public clang::PPCallbacks {
public:
  explicit SilenceRulesDiagnostics(clang::CompilerInstance &CI) : CI_(CI) {}

  void LexedFileChanged(clang::FileID FID, LexedFileChangeReason Reason,
                        clang::SrcMgr::CharacteristicKind FileType,
                        clang::FileID PrevFID,
                        clang::SourceLocation Loc) override {
    auto &src_mgr = CI_.getSourceManager();
    if (Reason == LexedFileChangeReason::EnterFile) {
      if (auto file = src_mgr.getFileEntryRefForID(FID);
          file && IsRulesEpilogue(file->getName())) {
        CI_.getDiagnostics().setSuppressAllDiagnostics(true);
      }
    }
    if (Reason == LexedFileChangeReason::ExitFile) {
      if (auto file = src_mgr.getFileEntryRefForID(PrevFID);
          file && IsRulesEpilogue(file->getName())) {
        CI_.getDiagnostics().setSuppressAllDiagnostics(false);
      }
    }
  }

private:
  clang::CompilerInstance &CI_;
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

  bool BeginSourceFileAction(clang::CompilerInstance &CI) override {
    CI.getPreprocessor().addPPCallbacks(
        std::make_unique<SilenceRulesDiagnostics>(CI));
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
