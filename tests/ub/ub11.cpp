// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic-ub: refcount
// nondet-result: unsafe
int main() {
  int *element = new int(10);
  int *ptr = element + 1;
  int out = *ptr;
  delete element;
  return out;
}
