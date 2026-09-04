// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <cstddef>

int main() {
  int b = 2;

  int *b_ptr = &b;
  *b_ptr = 3;
  // assert(b == 3);

  int **b_ptr_ptr = &b_ptr;
  **b_ptr_ptr = 4;
  *b_ptr = **b_ptr_ptr;
  // assert(b == 4);

  [[maybe_unused]] size_t offset = b_ptr - b_ptr;

  assert(b == 4);
  return 0;
}
