#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <string>
#include <vector>

static inline std::vector<std::string> getPlatformClangFlags() {
  std::vector<std::string> flags = {
      "-resource-dir=" CLANG_RESOURCE_DIR,
      "-I" COMPAT_INCLUDE_DIR,
      "-D_FORTIFY_SOURCE=0",
      "-Wno-gnu-include-next",
  };
#ifdef MACOS_SDK_PATH
  flags.push_back("-isysroot" MACOS_SDK_PATH);
#endif
  return flags;
}
