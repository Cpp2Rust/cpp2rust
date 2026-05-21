// no-compile: refcount
#include <assert.h>
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

int main(void) {
  test_setenv_getenv();
  return 0;
}
