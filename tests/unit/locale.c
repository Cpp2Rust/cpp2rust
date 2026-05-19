// no-compile: refcount
#include <assert.h>
#include <locale.h>
#include <stddef.h>
#include <string.h>

static void test_setlocale(void) {
  char *cur = setlocale(LC_ALL, NULL);
  assert(cur != NULL);
  assert(strcmp(cur, "C") == 0);
}

int main(void) {
  test_setlocale();
  return 0;
}
