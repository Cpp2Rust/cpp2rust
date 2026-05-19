// no-compile: refcount
#include <assert.h>
#include <errno.h>
#include <stdio.h>
#include <sys/stat.h>
#include <unistd.h>

static void test_stat(void) {
  const char *path = "/tmp/cpp2rust_stat_test.tmp";
  FILE *fp = fopen(path, "wb");
  assert(fp != NULL);
  fputs("hello", fp);
  assert(fclose(fp) == 0);
  struct stat st;
  assert(stat(path, &st) == 0);
  assert(st.st_size == 5);
  unlink(path);
}

static void test_fstat(void) {
  const char *path = "/tmp/cpp2rust_fstat_test.tmp";
  FILE *fp = fopen(path, "wb");
  assert(fp != NULL);
  fputs("hello world", fp);
  fflush(fp);
  int fd = fileno(fp);
  struct stat st;
  assert(fstat(fd, &st) == 0);
  assert(st.st_size == 11);
  assert(fclose(fp) == 0);
  unlink(path);
}

static void test_mkdir(void) {
  const char *path = "/tmp/cpp2rust_mkdir_test_dir";
  rmdir(path);
  assert(mkdir(path, 0755) == 0);
  struct stat st;
  assert(stat(path, &st) == 0);
  assert(S_ISDIR(st.st_mode));
  errno = 0;
  assert(mkdir(path, 0755) == -1);
  assert(errno == EEXIST);
  assert(rmdir(path) == 0);
  errno = 0;
}

int main(void) {
  test_stat();
  test_fstat();
  test_mkdir();
  return 0;
}
