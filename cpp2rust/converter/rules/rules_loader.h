// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#pragma once

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/Expr.h>
#include <clang/AST/Type.h>
#include <clang/Basic/SourceLocation.h>
#include <clang/Frontend/CompilerInstance.h>
#include <clang/Lex/Pragma.h>

#include <filesystem>
#include <string>

namespace cpp2rust::RulesLoader {

inline constexpr const char *kPragmaName = "cpp2rust_rules";

inline constexpr const char *kIndexDirName = "index";

std::filesystem::path ClassKey(const clang::NamedDecl *decl);

std::string MemberName(clang::DeclarationName name);

std::filesystem::path MemberKey(const std::filesystem::path &class_key,
                                const std::string &name);

std::filesystem::path FunctionKey(const clang::FunctionDecl *decl);

std::filesystem::path DeclKey(const clang::NamedDecl *decl);

std::filesystem::path ExprKey(clang::ASTContext &ctx, const clang::Expr *expr);

std::filesystem::path TypeKey(clang::QualType type);

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
