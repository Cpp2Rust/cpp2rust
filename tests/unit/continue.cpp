// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int main() {
  int out = 0;
  int i = 0;
  while (i < 10) {
    if (i % 2 == 0) {
      ++i;
      continue;
    }
    ++out;
    ++i;
  }
  for (int j = 0; j < 10; j++) {
    if (i % 2 == 0)
      continue;
    ++out;
  }
  for (int k1 = 0; k1 < 5; k1++) {
    for (int k2 = 0; k2 < 5; k2++) {
      int k3 = 0;
      while (k3 < 5) {
        if ((k1 + k2 + k3) % 2 == 0) {
          k3++;
          continue;
        }
        ++out;
        k3++;
      }
      if ((k1 + k2) % 2 == 0)
        continue;
      ++out;
    }
    if (k1 % 2 == 0)
      continue;
    ++out;
  }
  assert(out == 81);
  return 0;
}
