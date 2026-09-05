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

struct T {
  int v;
  auto operator<=>(const T &) const = default;
};

int main() {
  S a{1}, b{2};
  assert(a < b);
  assert(b > a);
  assert(a <= b);
  assert(b >= a);
  assert(a != b);
  assert((a <=> b) == std::strong_ordering::less);
  T x{3}, y{3};
  assert(x == y);
  assert(!(x < y));
  assert((x <=> y) == std::strong_ordering::equal);
  return 0;
}
