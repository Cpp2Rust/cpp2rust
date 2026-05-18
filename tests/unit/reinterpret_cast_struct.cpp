// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <cstdint>

struct Point {
  int32_t x;
  int32_t y;
};

// Reinterpret a struct as raw bytes.
// Fails because custom structs don't implement ByteRepr (yet!).
int main() {
  Point p;
  p.x = 0x04030201;
  p.y = 0x08070605;

  uint8_t *bytes = reinterpret_cast<uint8_t *>(&p);

  // Read x field bytes
  assert(bytes[0] == 0x01);
  assert(bytes[3] == 0x04);

  // Read y field bytes
  assert(bytes[4] == 0x05);
  assert(bytes[7] == 0x08);

  return 0;
}
