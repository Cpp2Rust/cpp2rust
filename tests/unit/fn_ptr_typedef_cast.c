#include <assert.h>
#include <stddef.h>

unsigned long call_with_ulong(unsigned long (*g)(unsigned long)) {
  return g(3) + 1;
}

unsigned long same_type(unsigned long a) { return a; }
unsigned long via_size_t_param(size_t b) { return b; }
size_t via_size_t_return(unsigned long b) { return b; }

struct pair {
  int a;
  int b;
};

unsigned long pair_scaled(struct pair p, size_t n) { return p.a * n + p.b; }
struct pair make_pair(size_t n) {
  struct pair p = {(int)n, (int)n * 2};
  return p;
}

int main() {
  assert(call_with_ulong(same_type) == 4);
  assert(call_with_ulong(via_size_t_param) == 4);
  assert(call_with_ulong((unsigned long (*)(unsigned long))via_size_t_return) ==
         4);

  typedef unsigned long (*ulong_fn)(unsigned long);
  typedef unsigned long (*size_t_fn)(size_t);
  size_t_fn original = via_size_t_param;
  ulong_fn adapted = (ulong_fn)original;
  size_t_fn back = (size_t_fn)adapted;
  assert(back == original);
  assert(back(5) == 5);

  typedef unsigned long (*pair_ulong_fn)(struct pair, unsigned long);
  pair_ulong_fn scaled = (pair_ulong_fn)pair_scaled;
  struct pair p = {3, 4};
  assert(scaled(p, 10) == 34);

  typedef struct pair (*make_ulong_fn)(unsigned long);
  make_ulong_fn make = (make_ulong_fn)make_pair;
  struct pair q = make(5);
  assert(q.a == 5);
  assert(q.b == 10);

  return 0;
}
