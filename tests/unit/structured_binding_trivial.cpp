// ADDITIONAL_COMPILE_FLAGS: -std=c++17
#include <assert.h>

struct Pair {
  int first;
  int second;
};

int main() {
  auto [in, fun] = Pair{1, 2};
  assert(in == 1);
  assert(fun == 2);

  Pair p{10, 20};
  auto [a, b] = p;
  a = 11;
  b += 1;
  assert(a == 11);
  assert(b == 21);
  assert(p.first == 10);
  assert(p.second == 20);
  return 0;
}
