#include <cassert>
#include <cstdint>

class Base {
public:
  Base(int16_t a, int8_t b) : a_(a), b_(b) {}

  int16_t a_;
  int8_t b_;
};

class Derived : public Base {
public:
  Derived(int16_t a, int8_t b, int8_t c) : Base(a, b), c_(c) {}

  int8_t c_;
};

int main() {
  Derived src(1, 2, 3);
  Derived dst(4, 5, 6);
  Base *s = static_cast<Base *>(&src);
  Base *t = static_cast<Base *>(&dst);
  *t = *s;
  assert(dst.a_ == 1);
  assert(dst.b_ == 2);
  assert(dst.c_ == 6);
  return 0;
}
