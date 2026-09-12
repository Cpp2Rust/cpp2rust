// ADDITIONAL_COMPILE_FLAGS: -std=c++20
#include <cassert>
#include <compare>

struct Eq {
  int a;
  int b;
  bool operator==(const Eq &) const = default;
};

struct Cmp {
  int a;
  int b;
  auto operator<=>(const Cmp &) const = default;
};

struct Both {
  int a;
  bool operator==(const Both &) const = default;
  std::strong_ordering operator<=>(const Both &) const = default;
};

int main() {
  Eq e1{1, 2}, e2{1, 2}, e3{1, 3};
  assert(e1 == e2);
  assert(e1 != e3);
  Cmp c1{1, 2}, c2{1, 3}, c3{2, 0}, c4{1, 9};
  assert(c1 < c2);
  assert(c3 > c4);
  assert(c1 == c1);
  assert((c1 <=> c2) == std::strong_ordering::less);
  Both b1{1}, b2{2};
  assert(b1 < b2);
  assert(b2 == b2);
  return 0;
}
