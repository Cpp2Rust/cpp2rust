#include <cassert>

int report(int Err) { return Err + 1; }

int main() {
  int in = 123;
  assert(in == 123);
  int Err = 1;
  assert(report(Err) == 2);
  return 0;
}
