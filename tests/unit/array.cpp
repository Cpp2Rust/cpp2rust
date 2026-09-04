// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int main() {
  const int N = 5;
  int arr1[N] = {0, 0, 0, 0, 0};
  const int arr2[] = {1, 1, 1, 1, 1};
  for (int i = 0; i < N; ++i) {
    arr1[i] = i + arr2[i];
  }
  int fatorial = 1;
  for (int i = 0; i < N; ++i) {
    fatorial *= arr1[i];
  }
  assert(fatorial == 120);
  return 0;
}
