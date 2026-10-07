#include <array>
#include <cassert>
#include <initializer_list>
#include <vector>

void f(std::initializer_list<int> list) {}

struct Pair {
  int a;
  int b;
};

struct Holder {
  Pair p{0, 0};
  Holder &operator=(Pair &&o) {
    p = o;
    return *this;
  }
};

int sum(const Pair &p) { return p.a + p.b; }

int main() {
  int i1{3};
  int i2{};
  int carr1[] = {1, 2};
  int carr2[3] = {1};
  std::array<int, 3> arr = {1, 2, 3};
  std::vector<int> vec = {1, 2, 3};
  f({1, 2, 3, 4});

  Pair p{1, 2};
  Holder h;
  h = {std::move(p)};
  assert(h.p.a == 1 && h.p.b == 2);
  assert(sum({p}) == 3);

  return 0;
}
