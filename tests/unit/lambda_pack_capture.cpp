// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <assert.h>

int sum() { return 0; }

template <typename T, typename... U> int sum(T t, U... u) {
  return t + sum(u...);
}

template <typename... T> int by_value(T... t) {
  auto f = [t...]() { return sum(t...); };
  ((t = 0), ...);
  return f();
}

template <typename... T> int by_ref(T... t) {
  auto f = [&t...]() { ((t *= 2), ...); };
  f();
  return sum(t...);
}

template <typename... T> int init_pack(T... t) {
  auto f = [... xs = t + 1]() { return sum(xs...); };
  return f();
}

template <typename... T> int implicit(T... t) {
  return [=]() { return sum(t...); }();
}

int main() {
  assert(by_value() == 0);
  assert(by_value(1, 2, 3) == 6);
  assert(by_ref(1, 2, 3) == 12);
  assert(init_pack(1, 2, 3) == 9);
  assert(implicit(4, 5) == 9);
  return 0;
}
