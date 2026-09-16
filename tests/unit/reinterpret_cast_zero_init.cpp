#include <cassert>
#include <cstdint>

// Build a u32 from scratch by writing individual bytes.
int main() {
  uint32_t val = 0;
  uint8_t *bytes = reinterpret_cast<uint8_t *>(&val);

  // Write little-endian 0xDEADBEEF
  bytes[0] = 0xEF;
  bytes[1] = 0xBE;
  bytes[2] = 0xAD;
  bytes[3] = 0xDE;

  assert(val == 0xDEADBEEF);
  return 0;
}
