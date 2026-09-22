// XFAIL: unsafe
#include <assert.h>
#include <stddef.h>

unsigned long call_with_ulong(unsigned long (*g)(unsigned long)) {
  return g(3) + 1;
}

unsigned long same_type(unsigned long a) { return a; }
unsigned long via_size_t_param(size_t b) { return b; }
size_t via_size_t_return(unsigned long b) { return b; }

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

  return 0;
}
