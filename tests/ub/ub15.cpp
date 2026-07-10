// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic-ub: refcount
// nondet-result: unsafe
int main() {
  int *arr = new int[15];
  int *ptr = arr + 15;
  int out = *ptr;
  delete[] arr;
  return out;
}
