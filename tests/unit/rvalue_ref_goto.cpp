// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int main() {
  int &&r = 5;
  goto body;
body:
  assert(r == 5);
  return 0;
}
