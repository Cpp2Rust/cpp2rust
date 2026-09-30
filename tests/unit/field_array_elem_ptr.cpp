#include <cassert>

struct Point {
  int x;
  int y;
};

struct Shape {
  int id;
  int coords[4];
  Point points[3];
  int tail;
};

static int sum(const int *p, int n) {
  int s = 0;
  for (int i = 0; i < n; ++i) {
    s += p[i];
  }
  return s;
}

static void set_y(Point *p, int y) { p->y = y; }

int main() {
  Shape s = {1, {10, 20, 30, 40}, {{1, 2}, {3, 4}, {5, 6}}, 99};

  // Pointers to elements of an array field, and arithmetic on them.
  int *c = &s.coords[1];
  assert(*c == 20);
  *c = 21;
  assert(s.coords[1] == 21);
  c += 2;
  assert(*c == 40);
  assert(c - &s.coords[0] == 3);
  assert(c[-1] == 30);
  assert(sum(s.coords, 4) == 101);
  assert(sum(&s.coords[2], 2) == 70);

  // Pointers to elements of an array of structs, and to their fields.
  Point *p = &s.points[1];
  assert(p->x == 3);
  set_y(p + 1, 60);
  assert(s.points[2].y == 60);
  int *py = &s.points[0].y;
  *py = 7;
  assert(s.points[0].y == 7);
  int *px = &(p + 1)->x;
  *px += 50;
  assert(s.points[2].x == 55);

  // The same, through a pointer to the struct.
  Shape *sp = &s;
  int *d = sp->coords + 3;
  *d = 41;
  assert(s.coords[3] == 41);
  sp->points[1].y = sp->coords[0] + sp->points[0].x;
  assert(s.points[1].y == 11);
  Point *q = sp->points;
  q[2].x = 8;
  assert(sp->points[2].x == 8);

  // Copies of the struct copy the arrays.
  Shape t = s;
  t.coords[0] = 0;
  t.points[0].x = 0;
  assert(s.coords[0] == 10 && s.points[0].x == 1);
  assert(t.coords[1] == 21 && t.points[2].y == 60 && t.tail == 99);
  return 0;
}
