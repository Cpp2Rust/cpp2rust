#include <cassert>
#include <cstdint>

int main() {
  uint8_t arr[4] = {0x01, 0x02, 0x03, 0x04};
  uint16_t *words = reinterpret_cast<uint16_t *>(arr);

  // Read
  assert(words[0] == 0x0201);
  assert(words[1] == 0x0403);

  // Write through u16 view
  words[0] = 0xBBAA;

  // Verify u8 array changed
  assert(arr[0] == 0xAA);
  assert(arr[1] == 0xBB);
  assert(arr[2] == 0x03);
  assert(arr[3] == 0x04);

  // Create an unaligned uint16_t
  words = reinterpret_cast<uint16_t *>(arr + 1);

  // Read
  assert(words[0] == 0x03BB);

  // Write through u16 view
  words[0] = 0x0000;

  // Verify u8 array changed
  assert(arr[0] == 0xAA);
  assert(arr[1] == 0x00);
  assert(arr[2] == 0x00);
  assert(arr[3] == 0x04);

  return 0;
}
