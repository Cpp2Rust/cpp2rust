// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <assert.h>

typedef int (*handler_t)(int);

struct Handler {
  int tag;
  handler_t cb;
};

int double_it(int x) { return x * 2; }
int negate(int x) { return -x; }

int main() {
  Handler h1 = {1, double_it};
  Handler h2 = {2, negate};

  assert(h1.cb != nullptr);
  assert(h1.cb(5) == 10);
  assert(h2.cb(7) == -7);

  h1.cb = negate;
  assert(h1.cb(3) == -3);
  assert(h1.cb == h2.cb);

  return 0;
}
