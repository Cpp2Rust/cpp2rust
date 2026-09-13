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

struct OrdOnly {
  int a;
  auto operator<=>(const OrdOnly &) const = default;
  bool operator==(const OrdOnly &) const = delete;
};

struct Inner {
  int x;
  auto operator<=>(const Inner &) const = default;
};

struct Outer {
  Inner i;
  int y;
  auto operator<=>(const Outer &) const = default;
};

// Secondary comparison operators can be defaulted too; they are defined as
// rewrites of the primary ones: != as !(a == b), < and >= via (a <=> b).
struct Secondary {
  int a;
  bool operator==(const Secondary &) const = default;
  bool operator!=(const Secondary &) const = default;
  auto operator<=>(const Secondary &) const = default;
  bool operator<(const Secondary &) const = default;
  bool operator>=(const Secondary &) const = default;
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
  OrdOnly o1{1}, o2{2};
  assert(o1 < o2);
  assert((o2 <=> o1) == std::strong_ordering::greater);
  Outer x1{{1}, 9}, x2{{2}, 0}, x3{{1}, 9};
  assert(x1 < x2);
  assert(x1 == x3);
  assert((x2 <=> x1) == std::strong_ordering::greater);
  Secondary s1{1}, s2{2};
  assert(s1 != s2);
  assert(s1 < s2);
  assert(s2 >= s1);
  assert(!(s2 < s1));
  return 0;
}
