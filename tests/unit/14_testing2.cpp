// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

int main() {
  int v = 1;
  int *ptr = &v;
  assert(*ptr == 1);
  return 0;
}
