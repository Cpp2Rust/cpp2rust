// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <assert.h>
#include <limits>

int main() {
  unsigned int x = -1;
  assert(x == std::numeric_limits<unsigned int>::max());
  [[maybe_unused]] int v1 = x & 1;
  [[maybe_unused]] unsigned v2 = x & 1;
  unsigned int *p = &x;
  [[maybe_unused]] bool b = *p & 255;
  int a = *p & 255;
  assert(a == 255);
  return 0;
}
