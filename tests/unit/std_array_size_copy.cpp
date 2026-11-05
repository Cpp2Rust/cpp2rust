#include <array>
#include <cassert>
#include <cstddef>

static int copies = 0;

struct Foo {
  int value = 0;
  Foo() = default;
  Foo(const Foo &o) : value(o.value) { ++copies; }
};

static std::size_t count(const std::array<Foo, 3> &a) { return a.size(); }

static int sum_of_copy(const std::array<Foo, 3> &a) {
  std::array<Foo, 3> b(a);
  return b[0].value + b[2].value;
}

int main() {
  std::array<Foo, 3> a{};
  a[0].value = 1;
  a[2].value = 4;
  assert(a.size() == 3 && count(a) == 3);
  assert(sum_of_copy(a) == 5 && copies == 3);
  std::array<Foo, 3> c(a);
  c[0].value = 7;
  assert(copies == 6 && a[0].value == 1 && c[0].value == 7);
  return 0;
}
