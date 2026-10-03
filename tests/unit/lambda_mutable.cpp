#include <assert.h>

int main() {
  int start = 5;
  auto next = [start]() mutable { return start++; };
  assert(next() == 5);
  assert(next() == 6);
  assert(next() == 7);
  assert(start == 5);

  int total = 0;
  auto accumulate = [total](int x) mutable {
    total += x;
    return total;
  };
  assert(accumulate(1) == 1);
  assert(accumulate(2) == 3);
  assert(total == 0);

  return 0;
}
