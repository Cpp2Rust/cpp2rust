// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int only_default(int x) {
  int r = 0;
  switch (x) {
  default:
    r = 42;
    break;
  }
  return r;
}

int main() {
  assert(only_default(1) == 42);
  return 0;
}
