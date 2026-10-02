// ADDITIONAL_COMPILE_FLAGS: -std=c++23

#include <cassert>

struct S {
  int v;
  operator char() const { return v; }
  operator char8_t() const { return v + 1; }
};

struct T {
  int v;
  operator long() const { return v; }
  operator long long() const { return v + 1; }
};

int main() {
  S s{65};
  char a = s;
  char8_t b = s;
  assert(a == 'A');
  assert(b == u8'B');
  assert(static_cast<char>(s) + static_cast<char8_t>(s) == 131);

  T t{3};
  long c = t;
  long long d = t;
  assert(c == 3);
  assert(d == 4);
  return 0;
}
