// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic-ub: refcount
// nondet-result: unsafe
int main() {
  int *arr = new int[10];
  int out = arr[10];
  delete[] arr;
  return out;
}
