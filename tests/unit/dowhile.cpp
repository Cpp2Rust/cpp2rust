// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int dowhile(int x) {
  do {
    x += 1;
    do {
      x += 1;
      x += 1;
    } while (x <= 100);
    x += 1;
  } while (x <= 200);
  return x;
}

int main() {
  assert(dowhile(0) == 202);
  return 0;
}
