#include <cassert>
#include <cstdint>

// Iterate over a u64 array viewed as u16 using a loop.
int main() {
  uint64_t arr[2] = {0x0004000300020001ULL, 0x0008000700060005ULL};
  uint16_t *words = reinterpret_cast<uint16_t *>(arr);

  for (int i = 0; i < 8; i++) {
    assert(words[i] == (uint16_t)(i + 1));
  }

  return 0;
}
