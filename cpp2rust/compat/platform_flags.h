#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <string>
#include <vector>

static inline std::vector<std::string> getPlatformClangBeginFlags() {
  std::vector<std::string> flags = {
      "-resource-dir=" CLANG_RESOURCE_DIR,
      "-isystem" COMPAT_INCLUDE_DIR,
      "-D_FORTIFY_SOURCE=0",
  };
#ifdef MACOS_SDK_PATH
  flags.push_back("-isysroot" MACOS_SDK_PATH);
#endif
  return flags;
}

static inline std::vector<std::string> getPlatformClangEndFlags() {
  return {
      "-Wno-gnu-include-next",
  };
}
