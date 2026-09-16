#include <cassert>
#include <cstdint>
#include <vector>

// Reinterpret a vector's data as u8, modify bytes, verify vector changed.
int main() {
  std::vector<uint32_t> vec;
  vec.push_back(0x04030201);
  vec.push_back(0x08070605);

  uint8_t *bytes = reinterpret_cast<uint8_t *>(vec.data());

  // Read from first element
  assert(bytes[0] == 0x01);
  assert(bytes[1] == 0x02);
  assert(bytes[2] == 0x03);
  assert(bytes[3] == 0x04);

  // Read from second element
  assert(bytes[4] == 0x05);
  assert(bytes[7] == 0x08);

  // Write to second element through byte view
  bytes[4] = 0xFF;
  assert(vec[1] == 0x080706FF);

  return 0;
}
