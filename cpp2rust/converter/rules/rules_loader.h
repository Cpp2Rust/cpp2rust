// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#pragma once

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/Basic/SourceLocation.h>
#include <clang/Frontend/CompilerInstance.h>
#include <clang/Lex/Pragma.h>

#include <string>

namespace cpp2rust::RulesLoader {

inline constexpr const char *kPragmaName = "cpp2rust_rules";

inline constexpr const char *kIndexDirName = "index";

std::string IndexPath(bool is_type, const std::string &key);

std::string ClassKey(clang::ASTContext &ctx, const clang::NamedDecl *decl);

std::string MemberKey(const std::string &class_key, const std::string &name);

class PragmaHandler : public clang::PragmaHandler {
public:
  PragmaHandler(clang::CompilerInstance &CI, const std::string &rules_dir)
      : clang::PragmaHandler(kPragmaName), CI_(CI), rules_dir_(rules_dir) {}

  void HandlePragma(clang::Preprocessor &PP, clang::PragmaIntroducer introducer,
                    clang::Token &tok) override;

private:
  clang::CompilerInstance &CI_;
  const std::string &rules_dir_;
};

} // namespace cpp2rust::RulesLoader
