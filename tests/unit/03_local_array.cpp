// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int main() {
  int arr1[] = {1, 2};
  arr1[0] = 3;
  arr1[1] = 4;

  assert(arr1[0] + arr1[1] == 7);
  return 0;
}
