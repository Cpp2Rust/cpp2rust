// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic-ub: refcount
// nondet-result: unsafe
int *dangling() {
  int x = 1;
  int *p = &x;
  return p;
}
int main() {
  int *x = dangling();
  return *x;
}
