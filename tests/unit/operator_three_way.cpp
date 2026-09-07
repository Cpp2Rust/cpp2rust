// ADDITIONAL_COMPILE_FLAGS: -std=c++20
#include <cassert>
#include <compare>

struct S {
  int v;
  std::strong_ordering operator<=>(const S &o) const {
    if (v < o.v) {
      return std::strong_ordering::less;
    }
    if (v > o.v) {
      return std::strong_ordering::greater;
    }
    return std::strong_ordering::equal;
  }
  bool operator==(const S &o) const { return v == o.v; }
};

int main() {
  S a{1}, b{2};
  assert(a < b);
  assert(b > a);
  assert(a <= b);
  assert(b >= a);
  assert(a != b);
  assert((a <=> b) == std::strong_ordering::less);
  return 0;
}
