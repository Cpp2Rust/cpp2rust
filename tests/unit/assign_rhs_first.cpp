#include <cassert>

// C++17 evaluates the right operand of an assignment before the left one, so
// a value that moves a pointer is assigned through the moved pointer.

static int **g_cursor;

static int advance() {
  ++*g_cursor;
  return 10;
}

struct S {
  int *ptr;
};

static void by_ref(int *&r) { r[1] += advance(); }

int main() {
  int a[8] = {1, 2, 3, 4, 5, 6, 7, 8};
  int *q = a;
  g_cursor = &q;

  // The place stays borrowed while the value is computed, unless the value
  // is computed first.
  q[1] = advance();
  assert(q == a + 1 && a[2] == 10);
  q[1] += advance();
  assert(q == a + 2 && a[3] == 14);

  int **pq = &q;
  (*pq)[1] += advance();
  assert(q == a + 3 && a[4] == 15);
  (*pq)[1] = advance();
  assert(q == a + 4 && a[5] == 10);

  by_ref(q);
  assert(q == a + 5 && a[6] == 17);

  S s{a};
  g_cursor = &s.ptr;
  s.ptr[1] += advance();
  assert(s.ptr == a + 1 && a[2] == 20);
  S *sp = &s;
  sp->ptr[1] += advance();
  assert(s.ptr == a + 2 && a[3] == 24);

  // Writes through reinterpreted pointers reach the original memory.
  unsigned char *b = (unsigned char *)a;
  b[0] = 7;
  b[4] += 1;
  assert(a[0] == 7 && a[1] == 3);
  return 0;
}
