#include <cassert>
#include <cstdint>

// Cast between signed and unsigned integer pointers of different sizes.
int main() {
  // i32 -> u16: read halves of a negative number
  int32_t neg = -1; // 0xFFFFFFFF
  uint16_t *words = reinterpret_cast<uint16_t *>(&neg);
  assert(words[0] == 0xFFFF);
  assert(words[1] == 0xFFFF);

  // i64 -> i16: read quarters of a large negative
  int64_t neg64 = -256; // 0xFFFFFFFFFFFFFF00
  int16_t *quarters = reinterpret_cast<int16_t *>(&neg64);
  assert(quarters[0] == -256);
  assert(quarters[1] == -1);
  assert(quarters[2] == -1);
  assert(quarters[3] == -1);

  return 0;
}
