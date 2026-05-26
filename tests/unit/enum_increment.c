#include <assert.h>

enum color { RED, GREEN, BLUE, COLOR_LAST };

int main() {
  int count = 0;
  for (enum color c = RED; c < COLOR_LAST; c++) {
    count++;
  }
  assert(count == 3);

  enum color c = RED;
  assert(c++ == RED);
  assert(++c == BLUE);
  assert(c-- == BLUE);
  assert(--c == RED);
  return 0;
}
