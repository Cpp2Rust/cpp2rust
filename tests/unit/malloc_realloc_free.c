#include <assert.h>
#include <stdlib.h>

#define ALLOC_TESTS(MALLOC, FREE, REALLOC, CALLOC)                             \
  do {                                                                         \
    int *p = (int *)MALLOC(sizeof(int));                                       \
    *p = 42;                                                                   \
    assert(*p == 42);                                                          \
    FREE(p);                                                                   \
                                                                               \
    int *arr = (int *)MALLOC(4 * sizeof(int));                                 \
    for (int i = 0; i < 4; i++) {                                              \
      arr[i] = i * 10;                                                         \
    }                                                                          \
    assert(arr[0] == 0);                                                       \
    assert(arr[3] == 30);                                                      \
    FREE(arr);                                                                 \
                                                                               \
    int *grow = (int *)MALLOC(2 * sizeof(int));                                \
    grow[0] = 1;                                                               \
    grow[1] = 2;                                                               \
    grow = (int *)REALLOC(grow, 4 * sizeof(int));                              \
    grow[2] = 3;                                                               \
    grow[3] = 4;                                                               \
    assert(grow[0] == 1);                                                      \
    assert(grow[1] == 2);                                                      \
    assert(grow[2] == 3);                                                      \
    assert(grow[3] == 4);                                                      \
    FREE(grow);                                                                \
                                                                               \
    int *zeros = (int *)CALLOC(4, sizeof(int));                                \
    for (int i = 0; i < 4; i++) {                                              \
      assert(zeros[i] == 0);                                                   \
    }                                                                          \
    FREE(zeros);                                                               \
  } while (0)

int main() {
  ALLOC_TESTS(malloc, free, realloc, calloc);

  void *(*pmalloc)(size_t) = malloc;
  void (*pfree)(void *) = free;
  void *(*prealloc)(void *, size_t) = realloc;
  void *(*pcalloc)(size_t, size_t) = calloc;

  ALLOC_TESTS(pmalloc, pfree, prealloc, pcalloc);

  return 0;
}
