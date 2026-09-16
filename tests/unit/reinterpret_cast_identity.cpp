#include <cassert>
#include <cstdint>

// reinterpret_cast preserves pointer identity: casting to another type
// and back yields the original pointer.
int main() {
  uint32_t val = 42;
  uint32_t *original = &val;

  uint16_t *as_u16 = reinterpret_cast<uint16_t *>(original);
  uint32_t *back = reinterpret_cast<uint32_t *>(as_u16);

  assert(back == original);
  assert(*back == 42);

  // Same through a different chain
  uint8_t *as_u8 = reinterpret_cast<uint8_t *>(original);
  uint32_t *back2 = reinterpret_cast<uint32_t *>(as_u8);

  assert(back2 == original);
  assert(*back2 == 42);

  return 0;
}
