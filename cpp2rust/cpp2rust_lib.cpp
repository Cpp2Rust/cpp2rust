// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "cpp2rust_lib.h"

#include <clang/Tooling/ArgumentsAdjusters.h>
#include <clang/Tooling/CompilationDatabase.h>
#include <clang/Tooling/Tooling.h>

#include "compat/platform_flags.h"
#include "frontend_action.h"

namespace cpp2rust {
std::string TranspileSrc(std::string_view cc_code, Model model,
                         const std::vector<std::string_view> &cxx_flags,
                         const std::string &rules_dir) {
  auto tool_args = getPlatformClangFlags();
  tool_args.push_back("-fparse-all-comments");
  tool_args.insert(tool_args.end(), cxx_flags.begin(), cxx_flags.end());

  std::string rs_code;
  clang::tooling::runToolOnCodeWithArgs(
      std::make_unique<FrontendAction>(rs_code, model, true, rules_dir),
      cc_code, tool_args, "input.cpp", CLANG_EXECUTABLE);
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
      getPlatformClangFlags(), clang::tooling::ArgumentInsertPosition::BEGIN));

  std::string rs_code;
  FrontendActionFactory factory(rs_code, model, rules_dir);
  Tool.run(&factory);
  return rs_code;
}
} // namespace cpp2rust
