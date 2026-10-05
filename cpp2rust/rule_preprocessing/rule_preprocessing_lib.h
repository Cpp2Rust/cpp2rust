// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#pragma once

#include <llvm/ADT/StringRef.h>

#include <filesystem>
#include <string>

namespace cpp2rust {

bool IsRuleName(llvm::StringRef name);

void AppendToFile(const std::filesystem::path &path, const std::string &text);

void RemoveFilesNamed(const std::filesystem::path &dir,
                      const std::string &file_name);

} // namespace cpp2rust
