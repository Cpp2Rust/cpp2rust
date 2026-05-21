// no-compile: refcount
#include <assert.h>
#include <stdlib.h>

int main() {
  int *p = (int *)malloc(sizeof(int));
  *p = 42;
  assert(*p == 42);
  free(p);

  int *arr = (int *)malloc(4 * sizeof(int));
  for (int i = 0; i < 4; i++) {
    arr[i] = i * 10;
  }
  assert(arr[0] == 0);
  assert(arr[3] == 30);
  free(arr);

  int *grow = (int *)malloc(2 * sizeof(int));
  grow[0] = 1;
  grow[1] = 2;
  grow = (int *)realloc(grow, 4 * sizeof(int));
  grow[2] = 3;
  grow[3] = 4;
  assert(grow[0] == 1);
  assert(grow[1] == 2);
  assert(grow[2] == 3);
  assert(grow[3] == 4);
  free(grow);

  int *zeros = (int *)calloc(4, sizeof(int));
  for (int i = 0; i < 4; i++) {
    assert(zeros[i] == 0);
  }
  free(zeros);

  return 0;
}
