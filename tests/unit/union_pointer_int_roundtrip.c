#include <assert.h>
#include <stdint.h>

int main(void) {
  int arr[4] = {10, 20, 30, 40};

  union {
    int *p;
    uintptr_t bits;
  } u;

  u.p = &arr[1];
  u.bits += 2 * sizeof(int);
  int *q = u.p;

  assert(*q == 40);
  assert(q == &arr[3]);

  u.bits -= 3 * sizeof(int);
  assert(u.p == &arr[0]);
  assert(*u.p == 10);

  u.p = arr + 4;
  assert(u.p == arr + 4);

  return 0;
}
