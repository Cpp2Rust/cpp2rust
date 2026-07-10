// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic-ub: refcount
// nondet-result: unsafe
int main() {
  int *arr1 = new int[100];
  arr1[100] = 1;
  delete[] arr1;
  return 0;
}
