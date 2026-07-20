#include <assert.h>
#include <fcntl.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

int main(void) {
  const char *path = "/tmp/cpp2rust_fd_io_test.tmp";
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
  int fds[2];
  assert(pipe(fds) == 0);
  assert(write(fds[1], "ab", 2) == 2);
  char buf2[4];
  memset(buf2, 0, sizeof(buf2));
  assert(read(fds[0], buf2, sizeof(buf2)) == 2);
  assert(strcmp(buf2, "ab") == 0);
  assert(close(fds[1]) == 0);
  assert(read(fds[0], buf2, sizeof(buf2)) == 0);
  assert(close(fds[0]) == 0);
  int s = socket(AF_INET, SOCK_STREAM, 0);
  assert(s >= 0);
  assert(close(s) == 0);
  return 0;
}
