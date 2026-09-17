// panic: refcount
#include <cstdint>

// Reinterpret a single u8 as u32 - only 1 byte available, need 4.
int main() {
  uint8_t b = 0x42;
  uint32_t *p = reinterpret_cast<uint32_t *>(&b);
  uint32_t val = *p;
  (void)val;
  return 0;
}
