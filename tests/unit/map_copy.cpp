#include <cassert>
#include <map>

struct S {
  std::map<int, int> m;
};

int main() {
  S a;
  a.m[1] = 10;
  S b = a;
  b.m[1] = 20;
  assert(a.m[1] == 10);
  std::map<int, int> x;
  x[2] = 3;
  std::map<int, int> y = x;
  y[2] = 4;
  assert(x[2] == 3);
  return 0;
}
