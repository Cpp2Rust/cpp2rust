#include <cassert>

template <typename... Args> int sum(Args... args) { return (args + ... + 0); }

template <typename T, typename... Args> T first(T x, Args... args) {
  return x + sum(args...);
}

int main() {
  assert(sum(1, 2, 3) == 6);
  assert(sum(4, 5) == 9);
  assert(first(10, 1, 2) == 13);
  assert(first(10, 0) == 10);
  return 0;
}
