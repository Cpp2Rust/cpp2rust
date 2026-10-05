// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#pragma once

#include <clang/AST/ASTContext.h>
#include <clang/Basic/SourceLocation.h>
#include <clang/Basic/SourceManager.h>
#include <clang/Lex/PPCallbacks.h>
#include <llvm/Support/JSON.h>

#include <filesystem>
#include <memory>
#include <string>
#include <vector>

namespace cpp2rust {

struct RuleFileDecl {
  std::string name;
  std::string text;
  std::string pointee_key;
  bool name_from_macro;
};

struct RuleFile {
  std::vector<RuleFileDecl> decls;
  std::vector<clang::CharSourceRange> includes;
  std::vector<std::string> common_includes;
  std::string text_without_includes;
};

std::unique_ptr<clang::PPCallbacks>
MakeIncludeCollector(clang::SourceManager &sm, RuleFile &file);

void CollectRuleFile(clang::ASTContext &ctx, RuleFile &file);

void WriteIndex(const std::filesystem::path &index_dir,
                const std::string &dir_name, bool is_c,
                const llvm::json::Object &rules, const RuleFile &file);

} // namespace cpp2rust
