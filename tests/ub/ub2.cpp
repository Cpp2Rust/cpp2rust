// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic-ub: refcount
// nondet-result: unsafe
int *null() {
  int *p = nullptr;
  return p;
}
int main() {
  int *x = null();
  return *x;
}
