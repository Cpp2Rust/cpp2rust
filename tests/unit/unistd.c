// translation-fail: refcount
#include <assert.h>
#include <fcntl.h>
#include <stdio.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/types.h>
#include <unistd.h>

static void test_close(void) {
  int fds[2];
  assert(pipe(fds) == 0);
  assert(close(fds[0]) == 0);
  char buf[1];
  assert(read(fds[0], buf, 1) == -1);
  assert(close(fds[1]) == 0);
}

static void test_lseek(void) {
  const char *path = "/tmp/cpp2rust_lseek_test.tmp";
  FILE *fp = fopen(path, "wb");
  assert(fp != NULL);
  fputs("hello world", fp);
  assert(fclose(fp) == 0);
  fp = fopen(path, "rb");
  assert(fp != NULL);
  int fd = fileno(fp);
  assert(lseek(fd, 0, SEEK_END) == 11);
  assert(lseek(fd, 6, SEEK_SET) == 6);
  char buf[8] = {0};
  assert(read(fd, buf, 5) == 5);
  assert(memcmp(buf, "world", 5) == 0);
  assert(fclose(fp) == 0);
  unlink(path);
}

static void test_read(void) {
  const char *path = "/tmp/cpp2rust_read_test.tmp";
  FILE *fp = fopen(path, "wb");
  assert(fp != NULL);
  fputs("hello world", fp);
  assert(fclose(fp) == 0);
  fp = fopen(path, "rb");
  assert(fp != NULL);
  int fd = fileno(fp);
  char buf[16] = {0};
  assert(read(fd, buf, 16) == 11);
  assert(memcmp(buf, "hello world", 11) == 0);
  assert(fclose(fp) == 0);
  unlink(path);
}

static void test_unlink(void) {
  const char *path = "/tmp/cpp2rust_unlink_test.tmp";
  FILE *fp = fopen(path, "wb");
  assert(fp != NULL);
  assert(fclose(fp) == 0);
  assert(unlink(path) == 0);
  assert(unlink(path) == -1);
}

static void test_pipe(void) {
  int fds[2];
  assert(pipe(fds) == 0);
  const char *msg = "world";
  assert(write(fds[1], msg, 5) == 5);
  char buf[8] = {0};
  assert(read(fds[0], buf, 8) == 5);
  assert(memcmp(buf, msg, 5) == 0);
  assert(close(fds[1]) == 0);
  assert(read(fds[0], buf, 8) == 0);
  assert(close(fds[0]) == 0);
}

static void test_ftruncate(void) {
  const char *path = "/tmp/cpp2rust_ftruncate_test.tmp";
  FILE *fp = fopen(path, "wb");
  assert(fp != NULL);
  fputs("hello world", fp);
  fflush(fp);
  int fd = fileno(fp);
  assert(ftruncate(fd, 5) == 0);
  assert(fclose(fp) == 0);
  fp = fopen(path, "rb");
  assert(fp != NULL);
  fd = fileno(fp);
  assert(lseek(fd, 0, SEEK_END) == 5);
  assert(fclose(fp) == 0);
  unlink(path);
}

static void test_open(void) {
  const char *path = "/tmp/cpp2rust_open_test.tmp";
  int fd = open(path, O_WRONLY | O_CREAT | O_TRUNC, 0644);
  assert(fd >= 0);
  assert(write(fd, "hello world", 11) == 11);
  assert(close(fd) == 0);
  fd = open(path, O_RDONLY);
  assert(fd >= 0);
  char buf[16] = {0};
  assert(read(fd, buf, 16) == 11);
  assert(memcmp(buf, "hello world", 11) == 0);
  assert(close(fd) == 0);
  unlink(path);
}

static void test_fcntl(void) {
  const char *path = "/tmp/cpp2rust_fcntl_test.tmp";
  int fd = open(path, O_WRONLY | O_CREAT | O_TRUNC, 0644);
  assert(fd >= 0);
  int flags = fcntl(fd, F_GETFL);
  assert(flags != -1);
  assert(fcntl(fd, F_SETFL, flags | O_APPEND) == 0);
  assert((fcntl(fd, F_GETFL) & O_APPEND) != 0);
  assert(close(fd) == 0);
  unlink(path);
}

static void test_ioctl(void) {
  int fds[2];
  assert(pipe(fds) == 0);
  assert(write(fds[1], "abcd", 4) == 4);
  int navail = 0;
  assert(ioctl(fds[0], FIONREAD, &navail) == 0);
  assert(navail == 4);
  assert(close(fds[0]) == 0);
  assert(close(fds[1]) == 0);
}

static void test_isatty(void) { printf("%d\n", isatty(0)); }

static void test_geteuid(void) { printf("%u\n", geteuid()); }

static void test_gethostname(void) {
  char name[256];
  assert(gethostname(name, sizeof(name)) == 0);
  printf("%s\n", name);
}

int main(void) {
  test_close();
  test_lseek();
  test_read();
  test_unlink();
  test_pipe();
  test_ftruncate();
  test_open();
  test_fcntl();
  test_ioctl();
  test_isatty();
  test_geteuid();
  test_gethostname();
  return 0;
}
