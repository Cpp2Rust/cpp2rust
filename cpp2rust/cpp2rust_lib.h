#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <string>
#include <vector>

#include "converter/factory.h"

namespace cpp2rust {

std::string TranspileSrc(std::string_view cc_code, Model model,
                         const std::vector<std::string_view> &cxx_flags,
                         const std::string &rules_dir,
                         std::string_view filename);
std::string TranspileDir(std::string_view build_dir, Model model,
                         const std::string &rules_dir);

} // namespace cpp2rust
