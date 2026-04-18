#include <assert.h>

struct Point {
  int x;
  int y;
};

struct Line {
  struct Point start;
  struct Point end;
};

struct Node {
  int value;
  struct Node *next;
};

struct Container {
  struct Inner {
    int a;
    int b;
  } inner;
  enum Color { RED, GREEN, BLUE } color;
  int count;
};

int main() {
  struct Point p = {10, 20};
  assert(p.x == 10);
  assert(p.y == 20);

  struct Line l = {{1, 2}, {3, 4}};
  assert(l.start.x == 1);
  assert(l.end.y == 4);

  struct Node a = {1, 0};
  struct Node b = {2, &a};
  assert(b.next->value == 1);

  struct Container c = {{5, 6}, GREEN, 42};
  assert(c.inner.a == 5);
  assert(c.inner.b == 6);
  assert(c.color == GREEN);
  assert(c.count == 42);

  struct Container c2;
  c2.color = BLUE;
  assert(c2.color == 2);

  return 0;
}
