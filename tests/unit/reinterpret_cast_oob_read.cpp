// panic: refcount
#include <cstdint>

// Read past the end of a u32 viewed as u8
int main() {
  uint32_t val = 0x04030201;
  uint8_t *bytes = reinterpret_cast<uint8_t *>(&val);
  uint8_t x = bytes[4];
  (void)x;
  return 0;
}
