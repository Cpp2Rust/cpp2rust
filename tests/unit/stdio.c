// no-compile: refcount
#include <stdio.h>

static void test_fputc(void) {
  fputc('H', stdout);
  fputc('i', stdout);
  fputc('\n', stdout);
}

static void test_fputs(void) {
  fputs("hello", stdout);
  fputc('\n', stdout);
  const char *s = "from variable";
  fputs(s, stdout);
  fputc('\n', stdout);
  char buf[] = {'b', 'u', 'f', '\0'};
  fputs(buf, stdout);
  fputc('\n', stdout);
}

int main(void) {
  test_fputc();
  test_fputs();
  return 0;
}
