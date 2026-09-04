// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int main() {
  int out = 0;
  int arr[5] = {1, 2, 3, 4, 0};
  for (const int *ptr = &arr[0]; *ptr != 0; ++ptr)
    out += *ptr;
  for (int *ptr = &arr[1]; *ptr != 4; ptr++)
    out += *ptr;
  for (int *ptr = &arr[4]; *ptr != 1; ptr--)
    out += *ptr;
  for (const int *ptr = &arr[3]; *ptr != 2; --ptr)
    out += *ptr;
  for (int *ptr = &arr[0]; *ptr != 0; ptr = ptr + 1)
    out += *ptr;
  int *ptr = &arr[0];
  for (int i = 0; i < 5; ++i)
    out += ptr[i];
  assert(out == 51);
  return 0;
}
