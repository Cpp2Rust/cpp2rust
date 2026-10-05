// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "rule_preprocessing/rule_index.h"

#include <clang/AST/Decl.h>
#include <clang/AST/DeclTemplate.h>
#include <clang/Tooling/Transformer/SourceCode.h>
#include <llvm/ADT/STLExtras.h>
#include <llvm/Support/raw_ostream.h>

#include <cstdlib>
#include <format>
#include <optional>
#include <utility>

#include "converter/printer.h"
#include "converter/rules/matcher.h"
#include "converter/rules/rules_loader.h"
#include "rule_preprocessing/rule_preprocessing_lib.h"

namespace cpp2rust {

namespace {

namespace fs = std::filesystem;

class IncludeCollector : public clang::PPCallbacks {
public:
  IncludeCollector(clang::SourceManager &sm, RuleFile &file)
      : sm_(sm), file_(file) {}

  void
  InclusionDirective(clang::SourceLocation HashLoc,
                     const clang::Token &IncludeTok, llvm::StringRef FileName,
                     bool IsAngled, clang::CharSourceRange FilenameRange,
                     clang::OptionalFileEntryRef File,
                     llvm::StringRef SearchPath, llvm::StringRef RelativePath,
                     const clang::Module *SuggestedModule, bool ModuleImported,
                     clang::SrcMgr::CharacteristicKind FileType) override {
    if (!sm_.isInMainFile(HashLoc)) {
      return;
    }
    if (!File) {
      return;
    }
    auto main_file = sm_.getFileEntryRefForID(sm_.getMainFileID());
    auto common_dir = fs::weakly_canonical(main_file->getName().str())
                          .parent_path()
                          .parent_path() /
                      "common";
    auto included = fs::weakly_canonical(File->getName().str());
    if (included.parent_path() == common_dir) {
      file_.common_includes.push_back(included.string());
    }
  }

private:
  clang::SourceManager &sm_;
  RuleFile &file_;
};

std::string PointeeKey(clang::ASTContext &ctx, const clang::Decl *decl) {
  auto typedef_decl = llvm::dyn_cast<clang::TypedefNameDecl>(decl);
  if (auto alias = llvm::dyn_cast<clang::TypeAliasTemplateDecl>(decl)) {
    typedef_decl = alias->getTemplatedDecl();
  }
  if (!typedef_decl || !typedef_decl->getUnderlyingType()->isPointerType()) {
    return {};
  }
  auto type = typedef_decl->getUnderlyingType();
  while (type->isPointerType()) {
    type = type->getPointeeType();
  }
  return Matcher::TypeKey(Printer::ToString(ctx, type.getUnqualifiedType(),
                                            Printer::ScalarSugar::kPreserve));
}

std::string Wrap(const std::string &dir_name, bool is_c,
                 const std::vector<std::string> &names,
                 const std::string &text) {
  if (!is_c) {
    return std::format("namespace cpp2rust_rules_{} {{\n{}}}\n", dir_name,
                       text);
  }
  std::string out;
  for (const auto &name : names) {
    out += std::format("#define {0} cpp2rust_rules_{1}_{0}\n", name, dir_name);
  }
  out += text;
  for (const auto &name : names) {
    out += std::format("#undef {}\n", name);
  }
  return out;
}

} // namespace

std::unique_ptr<clang::PPCallbacks>
MakeIncludeCollector(clang::SourceManager &sm, RuleFile &file) {
  return std::make_unique<IncludeCollector>(sm, file);
}

void CollectRuleFile(clang::ASTContext &ctx, RuleFile &file) {
  auto &sm = ctx.getSourceManager();
  for (auto *decl : ctx.getTranslationUnitDecl()->decls()) {
    if (decl->isImplicit() ||
        !sm.isInMainFile(sm.getExpansionLoc(decl->getLocation()))) {
      continue;
    }
    RuleFileDecl rule;
    if (auto named = llvm::dyn_cast<clang::NamedDecl>(decl)) {
      rule.name = named->getQualifiedNameAsString();
    }
    rule.text = clang::tooling::getText(
                    clang::tooling::getAssociatedRange(*decl, ctx), ctx)
                    .str() +
                '\n';
    rule.pointee_key = PointeeKey(ctx, decl);
    file.decls.push_back(std::move(rule));
  }
}

void WriteIndex(const fs::path &index_dir, const std::string &dir_name,
                bool is_c, const llvm::json::Object &rules,
                const RuleFile &file) {
  auto file_name = dir_name + ".inc";
  fs::create_directories(index_dir);
  RemoveFilesNamed(index_dir, file_name);

  std::string includes;
  if (!file.common_includes.empty()) {
    includes = std::format("#ifndef CPP2RUST_RULES_{0}_INCLUDES\n"
                           "#define CPP2RUST_RULES_{0}_INCLUDES\n",
                           dir_name);
    for (const auto &include : file.common_includes) {
      includes += std::format("#include \"{}\"\n", include);
    }
    includes += "#endif\n";
  }

  for (const auto &decl : file.decls) {
    if (!IsRuleName(decl.name)) {
      llvm::errs() << "ERROR: declaration '" << decl.name << "' in rule dir "
                   << dir_name
                   << " is not a rule; move it to an included file\n";
      std::exit(EXIT_FAILURE);
    }
    const auto *value = rules.get(decl.name);
    if (!value) {
      continue;
    }
    std::optional<llvm::StringRef> src = value->getAsString();
    if (!src) {
      src = value->getAsObject()->getString("key");
    }
    bool is_type = decl.name[0] == 't';
    std::string key;
    if (!is_type) {
      key = Matcher::ExprKey(src->str());
    } else if (!decl.pointee_key.empty()) {
      key = decl.pointee_key;
    } else {
      key = Matcher::TypeKey(src->str());
    }
    AppendToFile(index_dir / RulesLoader::IndexPath(is_type, key) / file_name,
                 Wrap(dir_name, is_c, {decl.name}, includes + decl.text));
  }
}

} // namespace cpp2rust
