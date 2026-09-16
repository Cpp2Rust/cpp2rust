#include <cassert>
#include <cstdint>

// Two u32 views into the same u64. Write through one, read through the other.
int main() {
  uint64_t val = 0x0807060504030201ULL;
  uint32_t *view1 = reinterpret_cast<uint32_t *>(&val);
  uint32_t *view2 = reinterpret_cast<uint32_t *>(&val);

  view1[0] = 0xDDCCBBAA;
  assert(view2[0] == 0xDDCCBBAA);
  assert(val == 0x08070605DDCCBBAAULL);

  view2[1] = 0xFFEEDDCC;
  assert(view1[1] == 0xFFEEDDCC);
  assert(val == 0xFFEEDDCCDDCCBBAAULL);

  return 0;
}
