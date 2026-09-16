#include <cassert>
#include <cstdint>

// View double as u64 to inspect IEEE 754 bit pattern directly.
// 1.0 = 0x3FF0000000000000
int main() {
  double d = 1.0;
  uint64_t *bits = reinterpret_cast<uint64_t *>(&d);
  assert(*bits == 0x3FF0000000000000ULL);

  // Modify the double through its u64 representation
  // 3.14159265358979 ~= 0x400921FB54442D18
  *bits = 0x400921FB54442D18ULL;
  assert(d > 3.14 && d < 3.15);

  return 0;
}
