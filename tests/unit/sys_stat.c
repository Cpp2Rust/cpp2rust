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

static void test_st_mtime_access(void) {
  struct stat st;
  struct stat *sp = &st;
  st.st_mtime = 5;
  assert(st.st_mtime == 5);
  sp->st_mtime = 6;
  assert(sp->st_mtime == 6);
  st.st_mtime += 2;
  assert(st.st_mtime == 8);
  sp->st_mtime -= 1;
  assert(sp->st_mtime == 7);
  st.st_mtime++;
  ++sp->st_mtime;
  assert(st.st_mtime == 9);
  time_t *mtime = &st.st_mtime;
  *mtime = 10;
  assert(st.st_mtime == 10);
  time_t *mtime_via_ptr = &sp->st_mtime;
  *mtime_via_ptr = 11;
  assert(*mtime == 11);
  time_t copy = sp->st_mtime;
  assert(copy == 11);
}

int main(void) {
  test_stat();
  test_fstat();
  test_st_mtime_access();
  return 0;
}
