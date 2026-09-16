#include <cassert>
#include <cstdint>

// Reinterpret a heap-allocated (new) value.
int main() {
  uint32_t *p = new uint32_t(0x04030201);
  uint8_t *bytes = reinterpret_cast<uint8_t *>(p);

  assert(bytes[0] == 0x01);
  assert(bytes[1] == 0x02);
  assert(bytes[2] == 0x03);
  assert(bytes[3] == 0x04);

  bytes[0] = 0x10;
  assert(*p == 0x04030210);

  delete p;
  return 0;
}
