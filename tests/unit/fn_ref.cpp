#include <cassert>

static int square(int x) { return x * x; }

static int twice(int x) { return 2 * x; }

static int call_ref(int (&f)(int), int x) { return f(x); }

template <class F> static int call_deduced(F &f, int x) { return f(x); }

template <class F> static int call_forwarded(F &&f, int x) { return f(x); }

struct Holder {
  int (&f)(int);
  int run(int x) const { return f(x); }
};

int main() {
  assert(call_ref(square, 3) == 9);
  assert(call_ref(twice, 3) == 6);

  int (&r)(int) = square;
  assert(r(4) == 16);
  assert(call_ref(r, 5) == 25);

  assert(call_deduced(twice, 7) == 14);
  assert(call_forwarded(square, 6) == 36);

  Holder h{twice};
  assert(h.run(8) == 16);
  return 0;
}
