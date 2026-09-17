#include <cassert>
#include <cstdint>

// View float as u32 to inspect IEEE 754 bit pattern directly.
// 1.0f = 0x3F800000, 2.0f = 0x40000000
int main() {
  float f = 1.0f;
  uint32_t *bits = reinterpret_cast<uint32_t *>(&f);
  assert(*bits == 0x3F800000);

  // Modify the float through its u32 representation
  *bits = 0x40000000;
  assert(f == 2.0f);

  return 0;
}
