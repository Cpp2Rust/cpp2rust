#include <assert.h>
#include <cstddef>
#include <cstdint>
#include <map>
#include <vector>

struct Degrees {
  int value;
  Degrees(int v) : value(v) {}
};

struct Reading {
  int raw;
  operator long() const { return raw * 10L; }
};

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

  uint8_t buf[5] = {10, 20, 30, 40, 50};
  const uint8_t *start = buf;
  size_t len = 5;
  std::vector<uint8_t> v5(start, start + len);
  assert(v5.size() == 5);
  assert(v5[0] == 10 && v5[4] == 50);

  std::map<int, int> m;
  m[1] = 10;
  m[2] = 20;
  m[3] = 30;
  std::vector<std::pair<int, int>> v6(m.begin(), m.end());
  assert(v6.size() == 3);
  assert(v6[0].first == 1 && v6[0].second == 10);
  assert(v6[2].first == 3 && v6[2].second == 30);
  v6.insert(v6.begin() + 1, m.begin(), m.end());
  assert(v6.size() == 6);
  assert(v6[1].first == 1 && v6[3].first == 3 && v6[4].first == 2);

  int ints[3] = {4, 5, 6};
  std::vector<Degrees> v7(ints, ints + 3);
  assert(v7[1].value == 5);

  Reading readings[2] = {{4}, {5}};
  std::vector<long> v8(readings, readings + 2);
  assert(v8[0] == 40L && v8[1] == 50L);

  return 0;
}
