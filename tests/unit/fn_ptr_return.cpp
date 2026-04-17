// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <assert.h>

typedef int (*op_t)(int);

int inc(int x) { return x + 1; }
int dec(int x) { return x - 1; }

op_t pick(int choose_inc) {
  if (choose_inc) {
    return inc;
  }
  return dec;
}

int main() {
  op_t f = pick(1);
  assert(f != nullptr);
  assert(f == inc);
  assert(f(10) == 11);

  op_t g = pick(0);
  assert(g == dec);
  assert(g(10) == 9);

  assert(f != g);
  return 0;
}
