// no-compile: refcount
#include <assert.h>
#include <errno.h>
#include <stdio.h>
#include <sys/stat.h>
#include <unistd.h>

static void test_stat(void) {
  const char *path = "cpp2rust_stat_test.tmp";
  FILE *fp = fopen(path, "wb");
  assert(fp != NULL);
  fputs("hello", fp);
  assert(fclose(fp) == 0);
  struct stat st;
  assert(stat(path, &st) == 0);
  assert(st.st_size == 5);
  assert(st.st_mtime > 0);
  unlink(path);
}

static void test_fstat(void) {
  const char *path = "cpp2rust_fstat_test.tmp";
  FILE *fp = fopen(path, "wb");
  assert(fp != NULL);
  fputs("hello world", fp);
  fflush(fp);
  int fd = fileno(fp);
  struct stat st;
  assert(fstat(fd, &st) == 0);
  assert(st.st_size == 11);
  assert(st.st_mtime > 0);
  assert(fclose(fp) == 0);
  unlink(path);
}

int main(void) {
  test_stat();
  test_fstat();
  return 0;
}
