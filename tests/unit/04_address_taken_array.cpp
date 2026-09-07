// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int main() {
  int arr2[] = {2, 2};
  arr2[0] = 3;
  arr2[1] = 4;

  int *arr2_ptr = arr2;
  arr2_ptr[0] = 5;
  arr2_ptr[1] = 6;

  int &arr2_ref1 = arr2[1];
  arr2_ref1 = 7;

  assert(arr2[0] + arr2[1] == 12);
  return 0;
}
