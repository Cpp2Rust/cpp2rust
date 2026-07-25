#include <assert.h>
#include <fcntl.h>
#include <string.h>
#include <unistd.h>

int main(void) {
  const char *path = "cpp2rust_fd_io_test.tmp";
  int fd = open(path, O_WRONLY | O_CREAT | O_TRUNC, 0644);
  assert(fd >= 0);
  assert(write(fd, "hello world", 11) == 11);
  assert(close(fd) == 0);
  fd = open(path, O_RDONLY);
  assert(fd >= 0);
  char buf[16];
  memset(buf, 0, sizeof(buf));
  assert(read(fd, buf, sizeof(buf)) == 11);
  assert(strcmp(buf, "hello world") == 0);
  assert(read(fd, buf, sizeof(buf)) == 0);
  assert(close(fd) == 0);
  assert(unlink(path) == 0);
  return 0;
}
