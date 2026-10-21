// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <filesystem>
#include <string>

using t1 = std::filesystem::path;

std::filesystem::path f1(const char *a0) { return std::filesystem::path(a0); }

std::string f2(const std::filesystem::path &a0) {
  return static_cast<std::string>(a0);
}
