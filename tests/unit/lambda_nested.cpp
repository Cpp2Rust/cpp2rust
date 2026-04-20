// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <assert.h>

int main() {
  int x = 10;

  auto outer = [&x](int y) {
    auto inner = [&x, y](int z) { return x + y + z; };
    return inner(1);
  };

  assert(outer(20) == 31);

  x = 100;
  assert(outer(20) == 121);

  return 0;
}
