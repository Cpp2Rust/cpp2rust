// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int for_test(const int n) {
  int x = 0;
  for (int i = 0, j = 0; i < n; j = i, i += 1) {
    x += 1;
    if (x == 100) {
      break;
    }
    for (int k = 0, w = 0; w < j; w += 1, k += 1, i += k)
      break;
    x = x + 1;
  }
  return x;
}

int main() {
  assert(for_test(200) == 400);
  return 0;
}
