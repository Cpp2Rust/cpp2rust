// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic-ub: refcount
// nondet-result: unsafe
int main() {
  int *arr = new int[10];
  int *ptr = arr + 1;
  int out = *ptr;
#pragma GCC diagnostic ignored "-Wfree-nonheap-object"
  delete[] ptr;
  return out;
}
