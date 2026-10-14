// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <string>

auto f6(std::string &s) -> decltype(s.data()) { return s.data(); }
