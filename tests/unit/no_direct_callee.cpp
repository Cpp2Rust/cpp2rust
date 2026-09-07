// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

bool test1() { return false; }

int test(decltype(test1) fn) {
  if (!fn()) {
    return 1;
  }
  return 0;
}

int main() {
  assert(test(test1) == 1);
  return 0;
}
