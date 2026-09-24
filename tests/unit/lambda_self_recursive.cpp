// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <assert.h>

int main() {
  auto fact = [](auto self, int n) -> int {
    if (n <= 1) {
      return 1;
    }
    return n * self(self, n - 1);
  };
  assert(fact(fact, 5) == 120);

  int calls = 0;
  auto fib = [&calls](auto &self, int n) -> int {
    calls++;
    if (n <= 2) {
      return 1;
    }
    return self(self, n - 1) + self(self, n - 2);
  };
  assert(fib(fib, 6) == 8);
  assert(calls == 15);

  int depth = 0;
  auto count_down = [&depth](this auto const &self, int n) -> void {
    if (n == 0) {
      return;
    }
    depth++;
    self(n - 1);
  };
  count_down(4);
  assert(depth == 4);

  return 0;
}
