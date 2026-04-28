#include <assert.h>

struct Outer {
  struct Named {
    int a;
    int b;
  } named;

  struct {
    int c;
    int d;
  } anon0;

  struct {
    int g;
    int h;
  } anon1;

  struct {
    int e;
    int f;
  };

  struct {
    int i;
    struct {
      int j;
    } inner_named;
    struct {
      int k;
    };
  };
};

int main(void) {
  struct Outer o = {0};

  o.named.a = 1;
  o.named.b = 2;
  o.anon0.c = 3;
  o.anon0.d = 4;
  o.anon1.g = 5;
  o.anon1.h = 6;
  o.e = 7;
  o.f = 8;
  o.i = 9;
  o.inner_named.j = 10;
  o.k = 11;

  assert(o.named.a == 1);
  assert(o.named.b == 2);
  assert(o.anon0.c == 3);
  assert(o.anon0.d == 4);
  assert(o.anon1.g == 5);
  assert(o.anon1.h == 6);
  assert(o.e == 7);
  assert(o.f == 8);
  assert(o.i == 9);
  assert(o.inner_named.j == 10);
  assert(o.k == 11);

  struct {
    int x;
    int z;
  } s;

  s.x = 1;
  s.z = 2;

  assert(s.x = 1);
  assert(s.z = 2);

  return 0;
}
