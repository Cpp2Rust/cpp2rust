#include <cassert>
#include <cstdint>

// Chain: u64 -> u32 -> u16, read and write at each level.
int main() {
  uint64_t val = 0x0807060504030201ULL;

  uint32_t *dwords = reinterpret_cast<uint32_t *>(&val);
  assert(dwords[0] == 0x04030201);
  assert(dwords[1] == 0x08070605);

  uint16_t *words = reinterpret_cast<uint16_t *>(dwords);
  assert(words[0] == 0x0201);
  assert(words[1] == 0x0403);
  assert(words[2] == 0x0605);
  assert(words[3] == 0x0807);

  // Write through u16 view
  words[1] = 0xBBAA;
  assert(dwords[0] == 0xBBAA0201);
  assert(val == 0x08070605BBAA0201ULL);

  // Write through u32 view
  dwords[1] = 0xFFEEDDCC;
  assert(val == 0xFFEEDDCCBBAA0201ULL);

  // Verify u16 view sees the u32 write
  assert(words[2] == 0xDDCC);
  assert(words[3] == 0xFFEE);

  return 0;
}
