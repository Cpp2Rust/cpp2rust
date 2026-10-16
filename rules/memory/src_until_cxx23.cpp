// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstddef>
#include <memory>

void f1(char *a0, std::size_t a1) { return std::declare_no_pointers(a0, a1); }

void f2(char *a0, std::size_t a1) {
  return std::undeclare_no_pointers(a0, a1);
}
