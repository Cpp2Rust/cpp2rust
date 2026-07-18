// no-compile: refcount
#include <assert.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>

static void test_setenv_getenv(void) {
  assert(setenv("CPP2RUST_TEST_VAR", "test_value", 1) == 0);
  const char *v = getenv("CPP2RUST_TEST_VAR");
  assert(v != NULL);
  assert(strcmp(v, "test_value") == 0);
  assert(setenv("CPP2RUST_TEST_VAR", "replaced", 1) == 0);
  v = getenv("CPP2RUST_TEST_VAR");
  assert(v != NULL);
  assert(strcmp(v, "replaced") == 0);
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
  test_setenv_getenv();
  test_realpath();
  return 0;
}
