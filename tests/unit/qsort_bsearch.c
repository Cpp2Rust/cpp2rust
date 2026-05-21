// no-compile: refcount
#include <assert.h>
#include <stdlib.h>
#include <string.h>

static int cmp_int(const void *a, const void *b) {
  int x = *(const int *)a;
  int y = *(const int *)b;
  return (x > y) - (x < y);
}

int main(void) {
  int arr[8] = {5, 2, 9, 1, 7, 3, 8, 4};
  qsort(arr, 8, sizeof(int), cmp_int);
  for (int i = 0; i < 7; ++i) {
    assert(arr[i] <= arr[i + 1]);
  }

  int key = 7;
  int *hit = (int *)bsearch(&key, arr, 8, sizeof(int), cmp_int);
  assert(hit != NULL);
  assert(*hit == 7);

  int miss_key = 42;
  int *miss = (int *)bsearch(&miss_key, arr, 8, sizeof(int), cmp_int);
  assert(miss == NULL);

  return 0;
}
