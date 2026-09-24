// no-compile
#include <assert.h>

int counter = 0;

auto inc = [](int x) { return x + 1; };
auto bump = []() {
  counter++;
  return counter;
};

template <typename F> int apply(F f, int x) { return f(x); }

int main() {
  assert(inc(41) == 42);

  bump();
  assert(bump() == 2);
  assert(counter == 2);

  assert(apply(inc, 1) == 2);

  decltype(inc) copy = inc;
  assert(copy(9) == 10);

  int (*fp)(int) = inc;
  assert(fp(-1) == 0);

  return 0;
}
