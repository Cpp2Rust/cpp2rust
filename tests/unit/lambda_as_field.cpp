// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// no-compile
#include <assert.h>
#include <utility>

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

template <typename F> struct Moving {
  F f;
  int moves = 0;
  Moving(F f) : f(f) {}
  Moving(Moving &&o) : f(std::move(o.f)), moves(o.moves + 1) {}
};

template <typename T> auto wrap(T fn) {
  return [fn](int x) { return fn(x) + 1; };
}

int count = 0;

int main() {
  {
    Guard g{[]() { count++; }};
    assert(count == 0);
  }
  assert(count == 1);

  Holder n{[](int x) { return x + 1; }, 0};
  assert(n.call(2) == 3);
  assert(n.calls == 1);

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

  Moving m{[](int x) { return x * 2; }};
  Moving m2(std::move(m));
  assert(m2.f(3) == 6);
  assert(m2.moves == 1);

  return 0;
}
