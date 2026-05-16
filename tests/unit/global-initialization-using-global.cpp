#include <assert.h>

int first;
int second = first + 1;

int main() {
  assert(first == 0);
  assert(second == first + 1);
  return 0;
}
