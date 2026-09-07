#include <cassert>
#include <utility>

int foo(int &&v) { return v; }

int main() {
  int &&i2 = 5;
  assert(i2 == 5);

  int i3 = foo(std::move(i2));
  assert(i3 == 5);
  assert(foo(5) == 5);
  return 0;
}
