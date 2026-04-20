#include <assert.h>
#include <stdbool.h>

int main() {
  int a = 0;
  int b;

  if (b = a) {
  }
  while ((b = a) != 0) {
  }
  if (a) {
  }
  if (a == b) {
  }
  if (a < b) {
  }

  assert(a == b);
  assert(!(a = b));

  bool c;
  c = a = b;
  c = (b = a) != 0;
  c = a;
  c = a == b;
  c = a < b;
  return 0;
}
