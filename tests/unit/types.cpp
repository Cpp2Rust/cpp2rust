// ADDITIONAL_COMPILE_FLAGS: -std=c++23

#include <cassert>
#include <cstddef>
#include <cstdint>
#include <inttypes.h>

int main() {
  const uint8_t xu8 = 8;
  const uint16_t xu16 = 16;
  uint32_t xu32 = 32;
  uint64_t xu64 = 64;
  const size_t xsz1 = 64;
  std::size_t xsz2 = 64;
  const int8_t xi1 = -8;
  const int16_t xi2 = 16;
  int32_t xi3 = 32;
  int64_t xi4 = 64;
  bool b = (xu64 == 64ULL);
  long double xld = 1.5;
  wchar_t xwc = 65;
  char8_t xc8 = 66;
  char16_t xc16 = 67;
  char32_t xc32 = 68;
  std::nullptr_t xnp = nullptr;
  assert(xu8 + xu16 + xu32 + xu64 + xsz1 + xsz2 + xi1 + xi2 + xi3 + xi4 == 352);
  assert(xld * 2 == 3);
  assert(xwc + xc8 + xc16 + xc32 == 266);
  assert(xnp == nullptr);
  return 0;
}
