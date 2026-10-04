// ADDITIONAL_COMPILE_FLAGS: -std=c++17
#include <assert.h>

struct Triple {
  int a;
  bool b;
  int c;
};

static int sum(const Triple &t) {
  const auto &[x, y, z] = t;
  return x + (y ? 1 : 0) + z;
}

int main() {
  Triple t{10, false, 20};
  auto &[pos, ins, nod] = t;
  pos = 11;
  ins = true;
  nod += 1;
  assert(t.a == 11);
  assert(t.b);
  assert(t.c == 21);

  t.a = 12;
  assert(pos == 12);

  const auto &[ca, cb, cc] = t;
  t.c = 30;
  assert(ca == 12);
  assert(cb);
  assert(cc == 30);

  assert(sum(t) == 43);
  return 0;
}
