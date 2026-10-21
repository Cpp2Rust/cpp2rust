// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <chrono>

using t1 = std::chrono::day;

std::chrono::day f1(unsigned a0) { return std::chrono::day(a0); }

unsigned f2(const std::chrono::day &a0) { return static_cast<unsigned>(a0); }
