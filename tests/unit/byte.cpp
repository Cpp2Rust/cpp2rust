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
  return 0;
}
