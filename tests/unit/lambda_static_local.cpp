#include <assert.h>

static int a;

int main() {
  auto next = []() {
    static int n = 0;
    return ++n;
  };
  assert(next() == 1);
  assert(next() == 2);

  auto copy = next;
  assert(copy() == 3);
  assert(next() == 4);

  auto per_type = [](auto x) {
    static int calls = 0;
    calls++;
    return calls;
  };
  assert(per_type(1) == 1);
  assert(per_type(2) == 2);
  assert(per_type(1.0) == 1);
  assert(per_type(3) == 3);

  auto read = [](int) { return a; };
  assert(read(0) == 0);
  a = 1;
  assert(read(0) == 1);

  auto local = []() {
    struct P {
      int x;
      int y;
    };
    P p = {2, 3};
    return p.x * p.y;
  };
  assert(local() == 6);

  return 0;
}
