// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// no-compile
#include <assert.h>

template <typename F> struct Guard {
  F f;
  ~Guard() { f(); }
};

template <typename F> struct Holder {
  F f;
  int calls;
  int call(int x) {
    calls++;
    return f(x);
  }
};

template <typename T> auto wrap(T fn) {
  return [fn](int x) { return fn(x) + 1; };
}

int main() {
  int cleaned = 0;
  {
    Guard g{[&cleaned]() { cleaned++; }};
    assert(cleaned == 0);
  }
  assert(cleaned == 1);

  int factor = 3;
  Holder h{[factor](int x) { return x * factor; }, 0};
  factor = 100;
  assert(h.call(2) == 6);
  assert(h.call(5) == 15);
  assert(h.calls == 2);

  auto w = wrap([factor](int x) { return x * factor; });
  factor = 7;
  assert(w(2) == 201);

  auto ww = wrap(w);
  assert(ww(2) == 202);

  return 0;
}
