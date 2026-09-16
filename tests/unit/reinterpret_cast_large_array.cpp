#include <cassert>
#include <cstdint>

// Access only the last element of a large u32 array viewed as u16.
int main() {
  const int N = 10000;
  uint32_t *arr = new uint32_t[N];
  for (int i = 0; i < N; i++) {
    arr[i] = 0;
  }
  arr[N - 1] = 0xBBAA9988;

  uint16_t *words = reinterpret_cast<uint16_t *>(arr);
  assert(words[N * 2 - 1] == 0xBBAA);
  assert(words[N * 2 - 2] == 0x9988);

  delete[] arr;
  return 0;
}
