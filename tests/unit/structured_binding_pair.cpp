// ADDITIONAL_COMPILE_FLAGS: -std=c++17
// no-compile
#include <assert.h>
#include <utility>

static std::pair<int, bool> make_std_pair(int x) { return {x, x > 0}; }

int main() {
  auto [value, positive] = make_std_pair(5);
  assert(value == 5);
  assert(positive);

  std::pair<int, int> p{1, 2};
  auto [a, b] = p;
  a = 10;
  assert(a == 10);
  assert(b == 2);
  assert(p.first == 1);

  auto &[x, y] = p;
  x = 3;
  y += 4;
  assert(p.first == 3);
  assert(p.second == 6);
  return 0;
}
