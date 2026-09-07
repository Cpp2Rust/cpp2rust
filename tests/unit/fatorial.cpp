// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int fatorial(int n) {
  if (n == 0)
    return 1;
  return n * fatorial(n - 1);
}

void fatorial_by_ref(int &n) {
  if (n == 1) {
    n *= 1;
    return;
  }
  int n_1 = n - 1;
  fatorial_by_ref(n_1);
  n *= n_1;
}

void fatorial_by_ptr(int *n) {
  if (*n == 1) {
    *n *= 1;
    return;
  }
  int n_1 = *n - 1;
  fatorial_by_ptr(&n_1);
  *n *= n_1;
}

int main() {
  int n1 = 2;
  fatorial_by_ptr(&n1);
  int n = n1 + 1;
  fatorial_by_ref(n);
  assert(fatorial(n) == 720);
  return 0;
}
