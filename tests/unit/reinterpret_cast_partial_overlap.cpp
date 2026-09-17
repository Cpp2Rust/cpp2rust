#include <cassert>
#include <cstdint>

// Access u32 array as u8 at an offset that crosses element boundaries.
int main() {
  uint32_t arr[2] = {0x04030201, 0x08070605};
  uint8_t *bytes = reinterpret_cast<uint8_t *>(arr);

  // Read bytes across the boundary of arr[0] and arr[1]
  assert(bytes[3] == 0x04); // last byte of arr[0]
  assert(bytes[4] == 0x05); // first byte of arr[1]

  // Write across the boundary: modify last byte of arr[0]
  bytes[3] = 0xFF;
  assert(arr[0] == 0xFF030201);

  // Write first byte of arr[1]
  bytes[4] = 0xAA;
  assert(arr[1] == 0x080706AA);

  return 0;
}
