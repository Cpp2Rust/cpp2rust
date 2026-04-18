// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <assert.h>

typedef void (*action_t)(int *);

void negate(int *x) { *x = -*x; }
void zero_out(int *x) { *x = 0; }

void run(action_t fn, int *x) { fn(x); }

int main() {
  int a = 42;
  run(negate, &a);
  assert(a == -42);

  run(zero_out, &a);
  assert(a == 0);

  action_t fn = negate;
  assert(fn != nullptr);
  int b = 10;
  fn(&b);
  assert(b == -10);

  return 0;
}
