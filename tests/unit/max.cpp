#include <algorithm>
#include <cassert>

static int calls = 0;
static int xs[2] = {1, 2};

int &pick(int i) {
  ++calls;
  return xs[i];
}

struct Bag {
  int n;
  Bag(int n) : n(n) {}
  Bag(const Bag &o) : n(o.n) { ++calls; }
};

bool operator<(const Bag &a, const Bag &b) { return a.n < b.n; }

Bag make(int n) { return Bag(n); }

int main() {
  int x1 = 1, x2 = 2, x3 = 10, x4 = 20;
  int *p1 = &x1, *p2 = &x2;
  int r1 = std::max(x1, x2);
  int r2 = std::min(x3, x4);
  int r3 = std::max(*p1, x2);
  int r4 = std::min(*p2, x3);
  int r5 = std::max(30, 40);
  assert(r1 + r2 + r3 + r4 + r5 == 56);
  int values[] = {1, 7, 7, 3};
  int *max = std::max_element(values, values + 4);
  assert(max == values + 1);

  // Each argument is evaluated once, and none is copied.
  assert(std::max(pick(0), pick(1)) == 2 && calls == 2);
  assert(std::min(pick(1), pick(0)) == 1 && calls == 4);
  calls = 0;
  assert(std::max(make(1), make(2)).n == 2 &&
         std::min(make(1), make(2)).n == 1);
  assert(calls == 0);
  return 0;
}
