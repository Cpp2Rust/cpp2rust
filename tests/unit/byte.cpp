// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <cstddef>

int main() {
  std::byte b1{0x01};

  int ishift1 = 3;
  std::byte shl1 = b1 << ishift1;
  assert(shl1 == std::byte(0x08));

  int ishift2 = 2;
  std::byte shr1 = shl1 >> ishift2;
  assert(shr1 == std::byte(0x02));

  int ishift3 = 5;
  b1 <<= ishift3;
  assert(b1 == std::byte(0x20));

  int ishift4 = 3;
  b1 >>= ishift4;
  assert(b1 == std::byte(0x04));

  std::byte b2{0x01};

  unsigned ushift1 = 3;
  std::byte shl2 = b2 << ushift1;
  assert(shl2 == std::byte(0x08));

  unsigned ushift2 = 2;
  std::byte shr2 = shl2 >> ushift2;
  assert(shr2 == std::byte(0x02));

  unsigned ushift3 = 5;
  b2 <<= ushift3;
  assert(b2 == std::byte(0x20));

  unsigned ushift4 = 3;
  b2 >>= ushift4;
  assert(b2 == std::byte(0x04));
  return 0;
}
