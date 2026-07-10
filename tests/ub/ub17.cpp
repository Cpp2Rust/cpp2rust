// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic-ub: refcount
// nondet-result: unsafe
int main() {
  int x = 1;
  int *p = &x;
#pragma GCC diagnostic ignored "-Wfree-nonheap-object"
  delete p;
  return 0;
}
