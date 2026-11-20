// no-compile: refcount
#include <assert.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>

static void test_getenv(void) {
  assert(getenv("CPP2RUST_DEFINITELY_MISSING_VAR") == NULL);
}

static void test_realpath(void) {
  char buf[4096];
  assert(realpath("/", buf) != NULL);
  assert(strcmp(buf, "/") == 0);
  char *p = realpath("/", NULL);
  assert(p != NULL);
  assert(strcmp(p, "/") == 0);
  free(p);
  errno = 0;
  assert(realpath("/cpp2rust_definitely_missing", buf) == NULL);
  assert(errno == ENOENT);
}

int main(void) {
  test_getenv();
  test_realpath();
  return 0;
}
