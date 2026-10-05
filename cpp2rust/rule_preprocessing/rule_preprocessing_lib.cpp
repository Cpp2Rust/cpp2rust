// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "rule_preprocessing/rule_preprocessing_lib.h"

#include <llvm/ADT/STLExtras.h>
#include <llvm/ADT/StringExtras.h>
#include <llvm/Support/FileSystem.h>
#include <llvm/Support/raw_ostream.h>

#include <cstdlib>
#include <system_error>
#include <vector>

namespace cpp2rust {

namespace fs = std::filesystem;

bool IsRuleName(llvm::StringRef name) {
  return (name.consume_front("f") || name.consume_front("t")) &&
         !name.empty() && llvm::all_of(name, llvm::isDigit);
}

void AppendToFile(const fs::path &path, const std::string &text) {
  fs::create_directories(path.parent_path());
  std::error_code ec;
  llvm::raw_fd_ostream out(path.string(), ec, llvm::sys::fs::OF_Append);
  if (ec) {
    llvm::errs() << "ERROR: failed to open " << path.string() << ": "
                 << ec.message() << '\n';
    std::exit(EXIT_FAILURE);
  }
  out << text;
}

void RemoveFilesNamed(const fs::path &dir, const std::string &file_name) {
  std::vector<fs::path> files;
  std::error_code ec;
  for (fs::recursive_directory_iterator it(dir, ec), end; !ec && it != end;
       it.increment(ec)) {
    if (it->path().filename() == file_name) {
      files.push_back(it->path());
    }
  }
  for (const auto &path : files) {
    fs::remove(path, ec);
  }
}

} // namespace cpp2rust
