// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "rules_dir.h"

#include <algorithm>
#include <array>
#include <filesystem>
#include <string>
#include <string_view>
#include <vector>

#include <llvm/Support/raw_ostream.h>

#if defined(_WIN32)
#include <windows.h>
#elif defined(__linux__)
#include <limits.h>
#include <unistd.h>
#elif defined(__APPLE__)
#include <mach-o/dyld.h>
#endif

namespace cpp2rust {

namespace fs = std::filesystem;

static fs::path GetExecutableDir() {
#if defined(_WIN32)
  char path[MAX_PATH];
  GetModuleFileNameA(NULL, path, MAX_PATH);
  return fs::path(path).parent_path();
#elif defined(__linux__)
  char path[PATH_MAX];
  ssize_t count = readlink("/proc/self/exe", path, PATH_MAX);
  return fs::path(std::string_view(path, std::max((ssize_t)0, count)))
      .parent_path();
#elif defined(__APPLE__)
  uint32_t size = 0;
  _NSGetExecutablePath(nullptr, &size);
  std::vector<char> buffer(size);
  _NSGetExecutablePath(buffer.data(), &size);
  return fs::path(buffer.data()).parent_path();
#endif
  return ".";
}

bool ResolveRulesDir(std::string &out) {
  std::array<fs::path, 3> candidates = {fs::path("./rules"),
                                        fs::path("../rules"),
                                        GetExecutableDir() / "../../rules"};

  for (const auto &dir : candidates) {
    if (fs::exists(dir) && fs::is_directory(dir)) {
      out = fs::canonical(dir).string();
      llvm::errs() << "Using rules directory: " << out << '\n';
      return true;
    }
  }
  llvm::errs() << "ERROR: rules directory not found. "
                  "Please specify one with --rules\n";
  return false;
}

} // namespace cpp2rust
