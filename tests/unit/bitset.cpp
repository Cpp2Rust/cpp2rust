// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <bitset>
#include <cassert>

int main() {
  std::bitset<16> b1(5);
  std::bitset<16> b2 = b1 << 2;
  std::bitset<16> b3 = b2 >> 1;

  assert(b1.to_ulong() == 5);
  assert(b2.to_ulong() == 20);
  assert(b3.to_ulong() == 10);

  return 0;
}
