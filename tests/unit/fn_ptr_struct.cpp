#include <assert.h>

typedef int (*handler_t)(int);

struct Handler {
  int tag;
  handler_t cb;
};

int double_it(int x) { return x * 2; }
int negate(int x) { return -x; }

struct S {
  static int pick(int x) { return x + 1; }
  static int pick(long x) { return (int)x + 2; }
  static int solo(int x) { return x + 3; }
};

int main() {
  handler_t p1 = &S::pick;
  handler_t p2 = S::solo;
  assert(p1(5) == 6);
  assert(p2(5) == 8);
  assert(S::pick(5L) == 7);

  Handler h3 = {3, &S::pick};
  assert(h3.cb(1) == 2);

  Handler h1 = {1, double_it};
  Handler h2 = {2, negate};

  assert(h1.cb != nullptr);
  assert(h1.cb(5) == 10);
  assert(h2.cb(7) == -7);

  h1.cb = negate;
  assert(h1.cb(3) == -3);
  assert(h1.cb == h2.cb);

  return 0;
}
