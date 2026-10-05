// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTConsumer.h>
#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/DeclTemplate.h>
#include <clang/Frontend/CompilerInstance.h>
#include <clang/Frontend/FrontendAction.h>
#include <clang/Tooling/Tooling.h>
#include <clang/Tooling/Transformer/SourceCode.h>
#include <llvm/ADT/STLExtras.h>
#include <llvm/ADT/SmallVector.h>
#include <llvm/ADT/StringRef.h>
#include <llvm/Support/CommandLine.h>
#include <llvm/Support/JSON.h>
#include <llvm/Support/MemoryBuffer.h>
#include <llvm/Support/raw_ostream.h>

#include <cstdlib>
#include <filesystem>
#include <format>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "compat/platform_flags.h"
#include "converter/converter_lib.h"
#include "converter/printer.h"
#include "converter/rules/matcher.h"
#include "converter/rules/rules_loader.h"
#include "rule_preprocessing/rule_preprocessing_lib.h"

namespace fs = std::filesystem;

namespace cpp2rust {

struct RuleFileDecl {
  std::string name;
  std::string text;
  std::string pointee_key;
};

using RuleFile = std::vector<RuleFileDecl>;

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
    file.push_back(std::move(rule));
  }
}

void WriteIndex(const fs::path &index_dir, const std::string &dir_name,
                bool is_c, const llvm::json::Object &rules,
                const RuleFile &file,
                const std::vector<fs::path> &common_headers) {
  auto file_name = dir_name + ".inc";
  fs::create_directories(index_dir);
  RemoveFilesNamed(index_dir, file_name);

  std::string includes;
  if (!is_c && !common_headers.empty()) {
    includes = std::format("#ifndef CPP2RUST_RULES_{0}_INCLUDES\n"
                           "#define CPP2RUST_RULES_{0}_INCLUDES\n",
                           dir_name);
    for (const auto &header : common_headers) {
      includes += std::format("#include \"{}\"\n", header.string());
    }
    includes += "#endif\n";
  }

  for (const auto &decl : file) {
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
  auto common_headers =
      cpp2rust::ListFiles(fs::weakly_canonical(dir).parent_path() / "common");
  for (const char *name : {"src.c", "src.cpp"}) {
    auto path = dir / name;
    if (!fs::exists(path)) {
      continue;
    }
    llvm::errs() << "Indexing " << path.string() << '\n';
    bool is_c = path.extension() == ".c";
    auto index_dir = ir_dir.parent_path() /
                     cpp2rust::RulesLoader::kIndexDirName /
                     (is_c ? "c" : "cpp");
    auto dir_name = ir_dir.filename().string();
    cpp2rust::RuleFile file;
    cpp2rust::Parse(path, file, cxx_flags);
    cpp2rust::WriteIndex(index_dir, dir_name, is_c, rules, file,
                         common_headers);
  }
  return EXIT_SUCCESS;
}
