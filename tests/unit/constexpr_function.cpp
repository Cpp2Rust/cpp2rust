#include <cassert>

int runtime_only(int x) { return x * 2; }

constexpr int first(const int *p) { return *p; }

constexpr int scaled(int x) {
  if (x < 0) {
    return runtime_only(-x);
  }
  return x;
}

constexpr double half(double x) { return x / 2.0; }

struct Flag {
  int v;
  constexpr explicit operator bool() const { return v != 0; }
};

constexpr int use(Flag f) {
  assert(f);
  return f.v;
}

constexpr int checked(int x) {
  assert(x > 0);
  return x + 1;
}

struct P {
  int v;
  constexpr int get() const { return v; }
};

int main() {
  int arr[2] = {7, 8};
  assert(first(arr) == 7);
  assert(first(arr + 1) == 8);
  assert(scaled(3) == 3);
  assert(scaled(-3) == 6);
  assert(half(5.0) == 2.5);
  assert(checked(1) == 2);
  constexpr int c = checked(4);
  assert(c == 5);
  assert(use(Flag{2}) == 2);
  constexpr int u = use(Flag{3});
  assert(u == 3);
  int *ptr = arr;
  assert(ptr);
  P p{9};
  assert(p.get() == 9);
  constexpr int k = scaled(4);
  assert(k == 4);
  return 0;
}
