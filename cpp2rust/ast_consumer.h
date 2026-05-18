#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTConsumer.h>
#include <clang/AST/ASTContext.h>
#include <clang/Frontend/CompilerInstance.h>

#include <string>

#include "converter/factory.h"

namespace cpp2rust {
class ASTConsumer : public clang::ASTConsumer {
public:
  explicit ASTConsumer(std::string &rs_code, Model model, bool first,
                       clang::CompilerInstance &CI,
                       const std::string &rules_dir)
      : rs_code_(rs_code), model_(model), first_(first), CI_(CI),
        rules_dir_(rules_dir) {}

  void HandleTranslationUnit(clang::ASTContext &ctx) override;

private:
  std::string &rs_code_;
  Model model_;
  bool first_;
  clang::CompilerInstance &CI_;
  const std::string &rules_dir_;
};
} // namespace cpp2rust
