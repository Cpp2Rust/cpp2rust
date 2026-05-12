#include <assert.h>
#include <stdlib.h>
int main() {
  int a = 0;
  int *p = &a;
  assert(p != NULL);
  assert(p != 0);

  p = NULL;
  assert(p == NULL);
  assert(p == 0);
  return 0;
}
