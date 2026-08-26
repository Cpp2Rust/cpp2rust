// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int &&g = 5;

int main() {
  assert(g == 5);
  return 0;
}
