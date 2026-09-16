#include <cassert>
#include <cstdint>

// Cast between i16 and u16 pointers (same size, different signedness).
int main() {
  int16_t vals[3] = {-1, 256, -32768};
  uint16_t *uvals = reinterpret_cast<uint16_t *>(vals);

  assert(uvals[0] == 0xFFFF); // -1 as unsigned
  assert(uvals[1] == 256);    // 256 same in both
  assert(uvals[2] == 0x8000); // -32768 as unsigned

  // Write through unsigned view
  uvals[0] = 42;
  assert(vals[0] == 42);

  return 0;
}
