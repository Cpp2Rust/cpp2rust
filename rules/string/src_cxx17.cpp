// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <filesystem>
#include <string>

typedef std::filesystem::path::string_type t8;

auto f6(std::string &s) -> decltype(s.data()) { return s.data(); }
