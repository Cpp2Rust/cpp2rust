#include <cassert>
#include <cstdint>

// Reinterpret a heap-allocated array.
int main() {
  uint32_t *arr = new uint32_t[2];
  arr[0] = 0x04030201;
  arr[1] = 0x08070605;

  uint8_t *bytes = reinterpret_cast<uint8_t *>(arr);

  assert(bytes[0] == 0x01);
  assert(bytes[4] == 0x05);
  assert(bytes[7] == 0x08);

  // Write through byte view
  bytes[0] = 0xAA;
  assert(arr[0] == 0x040302AA);

  bytes[5] = 0xBB;
  assert(arr[1] == 0x0807BB05);

  delete[] arr;
  return 0;
}
