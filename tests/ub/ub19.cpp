// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic-ub: refcount
// nondet-result: unsafe
void foo(int *array) { delete[] array; }
int main() {
  int *x = new int(1);
  foo(x);
  return 0;
}
