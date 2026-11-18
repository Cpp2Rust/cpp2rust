#include "h.h"

int b_count_int() { return count<int>(); }

int b_count_long() { return count<long>(); }

int b_count_local() {
  struct S {};
  return count<S>();
}
