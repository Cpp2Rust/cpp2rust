#include <cassert>
#include <cstdint>

// Cast i32 -> i16, modify halves, cast back to i32.
int main() {
  int32_t original = 0x04030201;
  int16_t *halves = reinterpret_cast<int16_t *>(&original);

  // Read through i16 view
  assert(halves[0] == 0x0201);
  assert(halves[1] == 0x0403);

  // Modify through i16 view
  halves[0] = 0x1010;

  // Verify original changed
  assert(original == 0x04031010);

  // Cast back: i16 array -> i32
  int16_t arr[2] = {0x0201, 0x0403};
  int32_t *as_int = reinterpret_cast<int32_t *>(arr);
  assert(*as_int == 0x04030201);

  return 0;
}
