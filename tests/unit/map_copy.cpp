#include <cassert>
#include <map>

typedef std::map<int, int> S;

int main() {
  S a;
  a[1] = 10;
  S b = a;
  b[1] = 20;
  assert(a[1] == 10);
  std::map<int, int> x;
  x[2] = 3;
  std::map<int, int> y = x;
  y[2] = 4;
  assert(x[2] == 3);
  return 0;
}
