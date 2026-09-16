#include <cassert>
#include <cstdint>

int main() {
  uint32_t value = 0x04030201;
  uint8_t *bytes = reinterpret_cast<uint8_t *>(&value);
  assert(bytes[0] == 0x01);
  assert(bytes[1] == 0x02);
  assert(bytes[2] == 0x03);
  assert(bytes[3] == 0x04);

  uint8_t arr[4] = {0x01, 0x02, 0x03, 0x04};
  uint16_t *arr16 = reinterpret_cast<uint16_t *>(arr);
  assert(arr16[0] == 0x0201);
  assert(arr16[1] == 0x0403);

  return 0;
}
