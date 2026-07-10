// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic-ub: refcount
// nondet-result: unsafe
void escape(int *ptr) { delete ptr; }

int main() {
  int *alloc = new int(1);
  escape(alloc);
  delete alloc;
  return 0;
}
