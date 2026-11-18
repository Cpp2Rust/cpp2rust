#include <cassert>

struct S {
  int x;
  int y;
};

struct W {
  int h;
  S arr[3];
};

struct P {
  int *a;
  int *arr[3];
};

// Pointers to the same object of the same type, one of them reached through
// the bytes of its allocation, whose elements have a different size in C
// than in Rust.
static void typed() {
  W w;
  S *p = &w.arr[2];
  S *q = (S *)((char *)&w + sizeof(int) + 2 * sizeof(S));
  assert(p == q);
  assert(&w.arr[1] < q);
  assert(q - &w.arr[0] == 2);

  int v = 0;
  int *ptrs[4] = {&v, &v, &v, &v};
  int **e = &ptrs[2];
  int **f = (int **)((char *)ptrs + 2 * sizeof(int *));
  assert(e == f);
  assert(&ptrs[1] < f);
  assert(f < &ptrs[3]);
  assert(f - ptrs == 2);

  P s;
  int **a = &s.a;
  int **b = &s.arr[1];
  assert(a < b);
  assert((int **)((char *)&s + 2 * sizeof(int *)) == b);
}

int main() {
  int i1 = 42;
  int *ptr1 = &i1;
  char *ptr2 = reinterpret_cast<char *>(&i1);

  void *vptr1 = ptr1;
  void *vptr2 = ptr2;
  assert(vptr1 == vptr2);

  typed();
  return 0;
}
