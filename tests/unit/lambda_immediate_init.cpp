#include <assert.h>

int pick(int x = [] { return 237; }()) { return x; }

struct S {
  int j = 10;
  int i = [this]() { return j * 2; }();
  int k = [] { return 3; }() + 1;
};

int g = [] {
  int s = 0;
  for (int i = 1; i <= 4; i++) {
    s += i;
  }
  return s;
}();

int main() {
  assert(pick() == 237);
  assert(pick(1) == 1);

  S s;
  assert(s.i == 20);
  assert(s.k == 4);

  S t{5};
  assert(t.i == 10);

  assert(g == 10);

  int a = 2;
  const int c = [&] {
    a++;
    return a * 10;
  }();
  assert(c == 30);
  assert(a == 3);

  return 0;
}
