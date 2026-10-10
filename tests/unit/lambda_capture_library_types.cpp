#include <assert.h>
#include <memory>
#include <utility>
#include <vector>

int main() {
  std::vector<int> v{1, 2};
  auto f = [v]() { return (int)v.size(); };
  auto g = f;
  auto h = std::move(f);
  assert(g() == 2);
  assert(h() == 2);
  assert(f() == 0);

  auto p = [u = std::make_unique<int>(5)]() { return u.get() ? *u : 0; };
  auto q = std::move(p);
  assert(q() == 5);
  assert(p() == 0);

  int n = 0;
  auto inner = [n]() mutable { return ++n; };
  auto outer = [inner]() mutable { return inner(); };
  assert(outer() == 1);
  auto outer2 = outer;
  assert(outer2() == 2);
  assert(outer() == 2);
  auto outer3 = std::move(outer);
  assert(outer3() == 3);

  return 0;
}
