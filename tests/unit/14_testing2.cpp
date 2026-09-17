#include <cassert>

int main() {
  int v = 1;
  int *ptr = &v;
  assert(*ptr == 1);
  return 0;
}
