// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "cpp2rust_lib.h"

#include <clang/Tooling/ArgumentsAdjusters.h>
#include <clang/Tooling/CompilationDatabase.h>
#include <clang/Tooling/Tooling.h>

#include <filesystem>

#include "compat/platform_flags.h"
#include "frontend_action.h"

namespace cpp2rust {
std::string TranspileSrc(std::string_view cc_code, Model model,
                         const std::vector<std::string_view> &cxx_flags,
                         const std::string &rules_dir,
                         std::string_view filename) {
  auto tool_args = getPlatformClangBeginFlags();
  tool_args.push_back("-fparse-all-comments");
  tool_args.insert(tool_args.end(), cxx_flags.begin(), cxx_flags.end());
  auto end_flags = getPlatformClangEndFlags();
  tool_args.insert(tool_args.end(), end_flags.begin(), end_flags.end());

  std::string rs_code;
  clang::tooling::runToolOnCodeWithArgs(
      std::make_unique<FrontendAction>(rs_code, model, true, rules_dir),
      cc_code, tool_args, std::filesystem::path(filename).filename().string(),
      filename.ends_with(".c") ? CLANG_C_COMPILER : CLANG_CXX_COMPILER);
  return rs_code;
}

std::string TranspileDir(std::string_view build_dir, Model model,
                         const std::string &rules_dir) {
  std::string error_message;
  auto compile_dbase = clang::tooling::CompilationDatabase::loadFromDirectory(
      build_dir, error_message);
  if (!compile_dbase) {
    return {};
  }

  std::vector<std::string> files;
  for (const auto &compile_command : compile_dbase->getAllCompileCommands()) {
    files.emplace_back(compile_command.Filename);
  }

  clang::tooling::ClangTool Tool(*compile_dbase, files);
  Tool.appendArgumentsAdjuster(clang::tooling::getInsertArgumentAdjuster(
      getPlatformClangBeginFlags(),
      clang::tooling::ArgumentInsertPosition::BEGIN));
  Tool.appendArgumentsAdjuster(clang::tooling::getInsertArgumentAdjuster(
      getPlatformClangEndFlags(), clang::tooling::ArgumentInsertPosition::END));
  // Redefine __FILE__ to use just the basename, so the generated code
  // doesn't contain system-specific absolute paths.
  Tool.appendArgumentsAdjuster(
      [](const clang::tooling::CommandLineArguments &args,
         llvm::StringRef filename) {
        auto result = args;
        auto basename =
            std::filesystem::path(filename.str()).filename().string();
        result.push_back("-Wno-builtin-macro-redefined");
        result.push_back("-D__FILE__=\"" + basename + "\"");
        return result;
      });

  std::string rs_code;
  FrontendActionFactory factory(rs_code, model, rules_dir);
  Tool.run(&factory);
  return rs_code;
}
} // namespace cpp2rust
