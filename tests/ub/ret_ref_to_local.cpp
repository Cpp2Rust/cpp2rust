// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic-ub: refcount

#include <cassert>

const int &foo() { return 5; }

int main() {
  int bar = foo();
  return 0;
}
