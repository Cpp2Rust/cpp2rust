#include <cassert>
#include <cstddef>

int main() {
  std::byte b1{0x01};

  unsigned ushift1 = 3;
  std::byte shl1 = b1 << ushift1;
  assert(shl1 == std::byte(0x08));

  unsigned ushift2 = 2;
  std::byte shr1 = shl1 >> ushift2;
  assert(shr1 == std::byte(0x02));

  unsigned ushift3 = 5;
  b1 <<= ushift3;
  assert(b1 == std::byte(0x20));

  unsigned ushift4 = 3;
  b1 >>= ushift4;
  assert(b1 == std::byte(0x04));

  int ishift = 1;
  std::byte shl2 = b1 << ishift;
  assert(shl2 == std::byte(0x08));

  std::byte b2{0x81};
  unsigned ushift5 = 8;
  std::byte shl3 = b2 << ushift5;
  assert(shl3 == std::byte(0x00));

  unsigned ushift6 = 9;
  std::byte shr2 = b2 >> ushift6;
  assert(shr2 == std::byte(0x00));

  unsigned ushift7 = 9;
  b2 <<= ushift7;
  assert(b2 == std::byte(0x00));

  std::byte b3{0x81};
  unsigned ushift8 = 8;
  b3 >>= ushift8;
  assert(b3 == std::byte(0x00));

  std::byte b4{0xff};
  int ishift2 = 12;
  std::byte shl4 = b4 << ishift2;
  assert(shl4 == std::byte(0x00));

  unsigned long lshift = 10;
  std::byte shl5 = b4 << lshift;
  assert(shl5 == std::byte(0x00));
  return 0;
}
