// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic-ub: refcount
// nondet-result: unsafe
int *smaller(int &x1, int &x2) { return (x1 < x2) ? &x1 : &x2; }
int main() {
  int *out = nullptr;
  int x1 = 1;
  if (x1) {
    int x2 = -1;
    out = smaller(x1, x2);
  }
  return *out;
}
