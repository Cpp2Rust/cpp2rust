#include <algorithm>
#include <cassert>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <vector>

enum E { A, B, C };

struct Pair {
  int x;
  int y;
};

struct alignas(64) Wide {
  char c;
};

struct Grid {
  int h;
  int row[3];
  int cells[2][3];
};

struct Bag {
  int n;
  std::vector<int> items;
};

// Two arrays whose pointers are cast to integers. The integer one past the
// end of `a` is not that of `b`, and the pointer it's cast to is moved back
// into `a` before it's accessed.
static void adjacent_arrays() {
  int a[4] = {1, 2, 3, 4};
  int b[4] = {5, 6, 7, 8};
  uintptr_t ia = (uintptr_t)a;
  uintptr_t ib = reinterpret_cast<uintptr_t>(b);
  assert(ia != ib);
  assert((uintptr_t)(a + 3) - ia == 3 * sizeof(int));
  assert((uintptr_t)&a[2] == ia + 2 * sizeof(int));

  int *end = (int *)(ia + sizeof(a));
  assert(end == a + 4);
  int *last = end - 1;
  assert(*last == 4);
  end[-2] = 30;
  assert(a[2] == 30);
  --end;
  *end = 40;
  assert(a[3] == 40);

  int *pb = reinterpret_cast<int *>(ib);
  pb[3] = 80;
  assert(b[3] == 80);
  assert(*(int *)(ib + sizeof(int)) == 6);

  // A pointer to the whole array is cast to the same integer.
  int (*pa)[4] = &a;
  assert((uintptr_t)pa == ia);
  assert(((int *)(uintptr_t)pa)[1] == 2);
}

// Comparisons of pointers cast from integers, with each other and with the
// pointers that were cast to the integers.
static void compare_physical() {
  int a[4] = {1, 2, 3, 4};
  int b[2] = {5, 6};
  uintptr_t ia = (uintptr_t)a;
  int *p1 = (int *)(ia + sizeof(int));
  int *p2 = (int *)(ia + 2 * sizeof(int));
  assert(p1 != p2);
  assert(p1 < p2);
  assert(p2 > p1);
  assert(p1 <= p1);
  assert(p1 + 1 == p2);
  assert(p2 - p1 == 1);
  assert(p1 == (int *)(ia + sizeof(int)));

  assert(p1 == &a[1]);
  assert(&a[2] == p2);
  assert(p1 != a);
  assert(p1 > a);
  assert(p2 < a + 3);
  assert(a + 4 == (int *)(ia + sizeof(a)));
  assert(p2 - a == 2);
  assert(a + 4 - p1 == 3);

  int *pb = (int *)(uintptr_t)b;
  assert(pb == b);
  assert(pb != p1);
  assert((void *)pb == (void *)b);
  assert((void *)pb != (void *)p1);
  assert(pb != nullptr);
  int *null = nullptr;
  assert((uintptr_t)null == 0);
  assert((int *)(uintptr_t)null == nullptr);
}

// Pointers cast from integers are ordered as the pointers they were cast
// from, also against pointers into other objects.
static void order_physical() {
  int x = 1, y = 2;
  int *px = &x, *py = &y;
  int *qx = (int *)(uintptr_t)px, *qy = (int *)(uintptr_t)py;
  assert(qx == px && !(qx < px) && !(px < qx));
  assert((px < py) == (qx < py));
  assert((px < py) == (px < qy));
  assert((px < py) == (qx < qy));
  assert((py < px) == (qy < qx));

  int *ps[4] = {py, qx, px, qy};
  std::sort(ps, ps + 4);
  assert(ps[0] == ps[1] && ps[2] == ps[3] && ps[1] < ps[2]);

  // An integer that is in no object is cast to a pointer that is no other.
  int *sentinel = (int *)(uintptr_t)-1;
  assert(sentinel != px && sentinel != nullptr);
  void *kept = (void *)(intptr_t)42;
  assert((intptr_t)kept == 42);
  assert(kept != (void *)px && kept != nullptr);
  assert((int *)(void *)(uintptr_t)px == px);
}

// The integers of pointers have the alignment of their pointees.
static void over_aligned() {
  char c = 'c';
  assert((uintptr_t)&c != 0);
  Wide w[3];
  uintptr_t iw = (uintptr_t)w;
  assert(iw % alignof(Wide) == 0);
  assert((uintptr_t)&w[1] - iw == sizeof(Wide));
  Wide *h = new Wide[2];
  assert((uintptr_t)h % alignof(Wide) == 0);
  delete[] h;
}

// Pointers into a vector keep their integers until it is reallocated.
static void vector_capacity() {
  std::vector<int> v;
  v.reserve(4);
  v.push_back(1);
  uintptr_t i0 = (uintptr_t)&v[0];
  v.push_back(2);
  v.push_back(3);
  assert((uintptr_t)&v[2] == i0 + 2 * sizeof(int));
  assert(*(int *)(i0 + sizeof(int)) == 2);
  v.reserve(100);
  uintptr_t j0 = (uintptr_t)&v[0];
  assert((uintptr_t)&v[2] - j0 == 2 * sizeof(int));
  assert(*(int *)(j0 + 2 * sizeof(int)) == 3);
}

// Pointers into arrays in structs and arrays, which are in the memory of
// their allocation, and into vectors that are not variables, whose elements
// are in a buffer of their own.
static void nested() {
  Grid g = {};
  uintptr_t ig = (uintptr_t)&g;
  assert((uintptr_t)&g.row[2] == ig + offsetof(Grid, row) + 2 * sizeof(int));
  int *pr = (int *)(ig + offsetof(Grid, row) + sizeof(int));
  *pr = 7;
  assert(g.row[1] == 7);
  assert(pr == &g.row[1]);
  uintptr_t cell = ig + offsetof(Grid, cells) + 4 * sizeof(int);
  assert((uintptr_t)&g.cells[1][1] == cell);
  *(int *)cell = 8;
  assert(g.cells[1][1] == 8);

  int m[3][4] = {};
  uintptr_t im = (uintptr_t)m;
  assert((uintptr_t)&m[2][1] == im + 9 * sizeof(int));
  int *pm = (int *)(im + 5 * sizeof(int));
  *pm = 42;
  assert(m[1][1] == 42);
  assert(pm == &m[1][1]);
  assert(pm < &m[2][0]);

  Bag b;
  b.items.reserve(4);
  b.items.push_back(1);
  b.items.push_back(2);
  uintptr_t ib = (uintptr_t)&b.items[0];
  assert((uintptr_t)&b.items[1] == ib + sizeof(int));
  b.items.push_back(3);
  assert((uintptr_t)&b.items[2] == ib + 2 * sizeof(int));
  *(int *)(ib + sizeof(int)) = 20;
  assert(b.items[1] == 20);
  assert((int *)ib == &b.items[0]);
  assert((uintptr_t)&b != ib);

  std::vector<std::vector<int>> vv(2, std::vector<int>(3, 5));
  uintptr_t iv = (uintptr_t)&vv[1][0];
  assert((uintptr_t)&vv[1][2] == iv + 2 * sizeof(int));
  assert((uintptr_t)&vv[0][0] != iv);
  *(int *)(iv + sizeof(int)) = 9;
  assert(vv[1][1] == 9);
  assert((int *)(iv + 2 * sizeof(int)) == &vv[1][2]);
}

static void struct_fields() {
  Pair pr = {1, 2};
  uintptr_t ip = (uintptr_t)&pr;
  assert((uintptr_t)&pr.y == ip + offsetof(Pair, y));
  int *py = (int *)(ip + offsetof(Pair, y));
  assert(*py == 2);
  *py = 20;
  assert(pr.y == 20);
  Pair *pp = (Pair *)ip;
  pp->x = 10;
  assert(pr.x == 10);
  assert(pp->y == 20);
  assert(pp == &pr);
}

static void heap() {
  int *h = new int[3]{1, 2, 3};
  uintptr_t ih = (uintptr_t)h;
  ((int *)ih)[2] = 9;
  assert(h[2] == 9);
  delete[] (int *)ih;

  long *l = (long *)malloc(2 * sizeof(long));
  l[1] = 7;
  long *back = (long *)(uintptr_t)(l + 1);
  assert(*back == 7);
  free((void *)((uintptr_t)back - sizeof(long)));
}

// C strings and memory accessed through pointers cast from integers.
static void c_strings() {
  char s[] = "hello";
  char *p = (char *)(uintptr_t)s;
  assert(strlen(p) == 5);
  assert(strcmp(p, "hello") == 0);
  char d[6];
  memcpy((char *)(uintptr_t)d, p, sizeof(s));
  assert(memcmp((char *)(uintptr_t)d, p, sizeof(s)) == 0);
  assert(strcmp(d, "hello") == 0);
}

int main() {
  size_t size = 1;
  assert(size == 1);

  unsigned long ul = 5;
  size_t s1 = size_t(ul);
  assert(s1 == 5);
  ul = (unsigned long)size_t(7);
  assert(ul == 7);

  int i = 2;
  E e = E(i);
  assert(e == C);
  assert(int(e) == 2);

  adjacent_arrays();
  compare_physical();
  order_physical();
  over_aligned();
  vector_capacity();
  nested();
  struct_fields();
  heap();
  c_strings();
  return 0;
}
