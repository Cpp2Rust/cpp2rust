// no-compile: refcount
#include <assert.h>
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

static void test_puts(void) {
  puts("puts hello");
  const char *s = "puts variable";
  puts(s);
}

static void test_fileno(void) {
  assert(fileno(stdin) == 0);
  assert(fileno(stdout) == 1);
  assert(fileno(stderr) == 2);
  const char* file = "/tmp/cpp2rust_fileno_test.tmp";
  FILE *fp = fopen(file, "wb");
  assert(fp != NULL);
  assert(fileno(fp) > 2);
  fclose(fp);
  // TODO: uncomment when unlink is available 
  // assert(unlink(file) == 0);
}

int main(void) {
  test_fputc();
  test_fputs();
  test_puts();
  test_fileno();
  return 0;
}
