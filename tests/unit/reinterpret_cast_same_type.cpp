#include <cassert>
#include <cstdint>

// Same-type reinterpret_cast (identity).
int main() {
  uint8_t arr[4] = {10, 20, 30, 40};
  uint8_t *same = reinterpret_cast<uint8_t *>(arr);

  assert(same[0] == 10);
  assert(same[1] == 20);
  assert(same[2] == 30);
  assert(same[3] == 40);

  // Write through the identity-cast pointer
  same[2] = 99;
  assert(arr[2] == 99);

  return 0;
}
