#include <assert.h>

struct S {
  int head;
  int tail[3];
  char buf[4];
};

struct S s = {5};

struct P {
  int x;
  int y;
};

int g[5] = {0};
static const int k[8] = {3, 0, 4};

static int counter = 0;
static int next_counter(void) { return ++counter; }

int main() {
  assert(s.head == 5);
  for (int i = 0; i < 3; i++) {
    assert(s.tail[i] == 0);
  }
  for (int i = 0; i < 4; i++) {
    assert(s.buf[i] == 0);
  }

  int a[10] = {0};
  for (int i = 0; i < 10; i++) {
    assert(a[i] == 0);
  }

  int seven[4] = {7};
  assert(seven[0] == 7);
  for (int i = 1; i < 4; i++) {
    assert(seven[i] == 0);
  }

  int b[6] = {1, 0, 2};
  assert(b[0] == 1 && b[1] == 0 && b[2] == 2);
  for (int i = 3; i < 6; i++) {
    assert(b[i] == 0);
  }

  double d[4] = {-0.0};
  assert(d[0] == 0.0 && d[3] == 0.0);

  struct P p[4] = {{1, 2}};
  assert(p[0].x == 1 && p[0].y == 2);
  assert(p[3].x == 0 && p[3].y == 0);

  int c[5] = {next_counter(), next_counter()};
  assert(c[0] + c[1] == 3 && c[4] == 0);

  int t[2] = {0};
  assert(t[0] == 0 && t[1] == 0);

  int u[3] = {5, 6};
  assert(u[0] == 5 && u[1] == 6 && u[2] == 0);

  assert(g[4] == 0);
  assert(k[0] == 3 && k[1] == 0 && k[2] == 4 && k[7] == 0);
  return 0;
}
