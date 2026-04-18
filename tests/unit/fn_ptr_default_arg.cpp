// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <assert.h>

typedef int (*transform_t)(int);

int identity(int x) { return x; }

int apply(int x, transform_t fn = nullptr) {
  if (fn) {
    return fn(x);
  }
  return x;
}

int main() {
  assert(apply(5) == 5);
  assert(apply(5, nullptr) == 5);
  assert(apply(5, identity) == 5);

  transform_t negate = [](int x) { return -x; };
  assert(apply(5, negate) == -5);

  return 0;
}
