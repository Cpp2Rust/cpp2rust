// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <assert.h>

typedef int (*op_t)(int);

int inc(int x) { return x + 1; }
int dec(int x) { return x - 1; }
int identity(int x) { return x; }

op_t pick(int mode) { return mode > 0 ? inc : mode < 0 ? dec : identity; }

int apply(op_t fn, int x) {
  op_t actual = fn ? fn : identity;
  return actual(x);
}

int main() {
  assert(pick(1)(10) == 11);
  assert(pick(-1)(10) == 9);
  assert(pick(0)(10) == 10);

  assert(apply(inc, 5) == 6);
  assert(apply(nullptr, 5) == 5);

  return 0;
}
