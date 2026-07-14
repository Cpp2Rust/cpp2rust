// panic: refcount
#include <assert.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void test_errno(void) {
  errno = 0;
  assert(errno == 0);
  errno = 42;
  assert(errno == 42);
  int saved = errno;
  assert(saved == 42);
  errno = 0;
}

static void test_errno_preserved_across_strdup(void) {
  errno = 99;
  char *d = strdup("hello");
  assert(d != NULL);
  assert(errno == 99);
  free(d);
  errno = 0;
}

static void test_errno_from_fseek(void) {
  errno = 0;
  int r = fseek(stdin, 0, SEEK_SET);
  assert(r == -1);
  assert(errno == ESPIPE);
  errno = 0;
}

int main(void) {
  test_errno();
  test_errno_preserved_across_strdup();
  test_errno_from_fseek();
  return 0;
}
