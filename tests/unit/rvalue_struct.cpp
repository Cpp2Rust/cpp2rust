#include <cassert>
#include <utility>

struct S {
  int a;
  int b;

  S(int a, int b) : a(a), b(b) {}
};

int main() {
  S s1(1, 2);
  S &&s2 = std::move(s1);
  assert(s2.a == 1);
  assert(s2.b == 2);
  return 0;
}
