// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic-ub: refcount

#include <cassert>

struct S {
  const int &r;
  S(const int &x) : r(x) {}
};

int main() {
  S s(5);
  assert(s.r == 5);
  return 0;
}
