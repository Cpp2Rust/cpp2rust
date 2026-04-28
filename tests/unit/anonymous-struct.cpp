#include <cassert>

struct Outer {
  struct Named {
    int a;
    int b;
  } named;

  struct {
    int c;
    int d;
  } anonymous_named_0;

  struct {
    int g;
    int h;
  } anonymous_named_1;

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

int main() {
  Outer o = {};

  o.named.a = 1;
  o.named.b = 2;
  o.anonymous_named_0.c = 3;
  o.anonymous_named_0.d = 4;
  o.anonymous_named_1.g = 5;
  o.anonymous_named_1.h = 6;
  o.e = 7;
  o.f = 8;
  o.i = 9;
  o.inner_named.j = 10;
  o.k = 11;

  assert(o.named.a == 1);
  assert(o.named.b == 2);
  assert(o.anonymous_named_0.c == 3);
  assert(o.anonymous_named_0.d == 4);
  assert(o.anonymous_named_1.g == 5);
  assert(o.anonymous_named_1.h == 6);
  assert(o.e == 7);
  assert(o.f == 8);
  assert(o.i == 9);
  assert(o.inner_named.j == 10);
  assert(o.k == 11);

  return 0;
}
