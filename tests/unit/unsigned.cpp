// ADDITIONAL_COMPILE_FLAGS: -std=c++20
#include <assert.h>
#include <bit>
#include <cstdint>
#include <limits>

int main() {
  unsigned int x = -1;
  assert(x == std::numeric_limits<unsigned int>::max());
  [[maybe_unused]] int v1 = x & 1;
  [[maybe_unused]] unsigned v2 = x & 1;
  unsigned int *p = &x;
  [[maybe_unused]] bool b = *p & 255;
  int a = *p & 255;
  assert(a == 255);

  assert(std::bit_width(0u) == 0 && std::bit_width(x) == 32);
  assert(std::bit_floor(0u) == 0 && std::bit_floor(100u) == 64);
  assert(std::bit_ceil(0u) == 1 && std::bit_ceil(100u) == 128);
  assert(std::has_single_bit(64u) && !std::has_single_bit(0u));
  uint8_t byte = 0x81;
  assert(std::rotl(byte, 1) == 0x03 && std::rotl(byte, -1) == 0xC0);
  assert(std::rotr(byte, 1) == 0xC0 && std::rotr(byte, 9) == 0xC0);
  assert(std::countl_zero(uint8_t{1}) == 7 && std::countl_one(byte) == 1);
  uint64_t wide = 0xF0ull;
  assert(std::countr_zero(wide) == 4 && std::countr_one(wide >> 4) == 4);
  assert(std::popcount(wide) == 4 && std::countl_zero(uint64_t{0}) == 64);
  uint8_t floor_byte = std::bit_floor(byte);
  uint64_t ceil_wide = std::bit_ceil(wide + 1);
  uint64_t rot_wide = std::rotl(wide, 60);
  assert(floor_byte == 0x80 && ceil_wide == 0x100 && rot_wide == 0xFull);
  unsigned long ul = (1ul << 40) | 1;
  assert(std::popcount(ul) == 2 && std::countl_zero(ul) == 23);
  assert(std::bit_width(ul) == 41 && std::bit_floor(ul) == 1ul << 40);
  assert(std::rotr(ul, 1) == ((1ul << 39) | (1ul << 63)));
  return 0;
}
