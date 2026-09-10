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
  P p{9};
  assert(p.get() == 9);
  constexpr int k = scaled(4);
  assert(k == 4);
  return 0;
}
