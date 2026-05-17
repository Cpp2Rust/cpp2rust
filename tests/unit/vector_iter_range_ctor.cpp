#include <assert.h>
#include <cstddef>
#include <cstdint>
#include <vector>

int main() {
  uint32_t src[3] = {1, 2, 3};

  // Same-type iterator-range constructor.
  std::vector<uint32_t> v1(src, src + 3);
  assert(v1.size() == 3);
  assert(v1[0] == 1 && v1[1] == 2 && v1[2] == 3);

  // Cross-type widening (uint32_t -> size_t).
  std::vector<size_t> v2(src, src + 3);
  assert(v2.size() == 3);
  assert(v2[0] == 1 && v2[1] == 2 && v2[2] == 3);

  // Cross-type sign-flip (uint32_t -> int).
  std::vector<int> v3(src, src + 3);
  assert(v3.size() == 3);
  assert(v3[0] == 1 && v3[1] == 2 && v3[2] == 3);

  const uint32_t src1[3] = {1, 2, 3};
  auto v4 = std::vector<uint32_t>(src1, std::end(src1));
  assert(v4.size() == 3);
  assert(v4[0] == 1 && v4[1] == 2 && v4[2] == 3);

  return 0;
}
