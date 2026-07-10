// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic-ub: refcount
// nondet-result: unsafe
void null(int **p) { *p = nullptr; }
int main() {
  int x = 1;
  int *p = &x;
  null(&p);
  int &r = *p;
  return r;
}
