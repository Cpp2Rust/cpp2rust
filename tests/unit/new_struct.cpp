// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

struct Pair {
  int x, y;
};

int main() {
  Pair *p = new Pair{1, 2};
  int out = p->x + p->y;
  delete p;
  assert(out == 3);
  return 0;
}
