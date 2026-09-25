// no-compile: refcount
#include <assert.h>

struct S {
  int v;
  int nested_this() {
    auto outer = [this](int y) {
      auto inner = [this, y](int z) { return v + y + z; };
      return inner(1);
    };
    return outer(20);
  }
};

int main() {
  int x = 10;

  auto outer = [&x](int y) {
    auto inner = [&x, y](int z) { return x + y + z; };
    return inner(1);
  };

  assert(outer(20) == 31);

  x = 100;
  assert(outer(20) == 121);

  S s = {5};
  assert(s.nested_this() == 26);

  return 0;
}
