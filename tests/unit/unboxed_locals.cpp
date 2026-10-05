#include <cassert>

static void set(int *p, int v) { *p = v; }

static void inc(int &r) { ++r; }

struct Pair {
  int first;
  int second;
};

static int sum(const int *arr, int n) {
  int s = 0;
  for (int i = 0; i < n; ++i) {
    s += arr[i];
  }
  return s;
}

// Parameters that are only read and written are modified in place.
static int countdown(int n, int step = 1) {
  int steps = 0;
  while (n > 0) {
    n -= step;
    steps++;
  }
  return steps;
}

struct Shape {
  virtual ~Shape() {}
  virtual int scale(int factor) = 0;
};

struct Square : Shape {
  int side = 2;
  int scale(int factor) override { return factor * side; }
};

int main() {
  // Unboxed: only its value is accessed.
  int plain = 1;
  plain += 2;
  plain++;
  assert(plain == 4);
  (void)sizeof(plain);

  // Boxed: its address is taken.
  int addr = 0;
  set(&addr, 5);
  assert(addr == 5);

  // Boxed: bound to a reference.
  int ref = 1;
  inc(ref);
  int &alias = ref;
  alias++;
  assert(ref == 3);

  // Boxed: captured by a lambda.
  int captured = 7;
  auto get = [&captured]() { return captured; };
  captured = 8;
  assert(get() == 8);

  // Unboxed: its elements are only read and written.
  int arr[4] = {1, 2, 3, 4};
  arr[0] = arr[3] * 2;
  arr[1]++;
  assert(arr[0] + arr[1] == 11);

  // Boxed: a pointer to one of its elements is made.
  int elem[2] = {0, 0};
  set(&elem[1], 6);
  assert(elem[1] == 6);

  // Boxed: it decays to a pointer.
  int decayed[3] = {1, 2, 3};
  assert(sum(decayed, 3) == 6);

  // Boxed: one of its elements is bound to a reference.
  int elem_ref[2] = {1, 1};
  inc(elem_ref[0]);
  assert(elem_ref[0] == 2);

  // Unboxed: pointers are values too.
  int *p = &addr;
  p = &elem[0];
  *p = 9;
  assert(elem[0] == 9);

  char str[8] = "abc";
  str[0] = 'x';
  assert(str[0] == 'x' && str[3] == '\0');

  double zeros[16] = {};
  zeros[15] = 1.5;
  assert(zeros[0] == 0 && zeros[15] == 1.5);

  assert(countdown(10) == 10);
  assert(countdown(10, 3) == 4);

  // Unboxed: read in an initializer list.
  int init = 3;
  Pair pair = {init, init + 1};
  Pair *heap = new Pair{init, init};
  assert(pair.second == 4 && heap->first == 3);
  delete heap;

  Square square;
  Shape *shape = &square;
  assert(shape->scale(3) == 6);
  return 0;
}
