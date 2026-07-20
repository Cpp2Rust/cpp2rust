#include <assert.h>
#include <fcntl.h>
#include <poll.h>
#include <string.h>
#include <sys/select.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <termios.h>
#include <unistd.h>

static void test_open_read_write(void) {
  const char *path = "/tmp/cpp2rust_fd_io_rw.tmp";
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
}

static void test_pipe(void) {
  int fds[2];
  assert(pipe(fds) == 0);
  assert(write(fds[1], "ab", 2) == 2);
  char buf[4];
  memset(buf, 0, sizeof(buf));
  assert(read(fds[0], buf, sizeof(buf)) == 2);
  assert(strcmp(buf, "ab") == 0);
  assert(close(fds[1]) == 0);
  assert(read(fds[0], buf, sizeof(buf)) == 0);
  assert(close(fds[0]) == 0);
}

static void test_socket_listen(void) {
  int s = socket(AF_INET, SOCK_STREAM, 0);
  assert(s >= 0);
  assert(listen(s, 5) == 0);
  assert(close(s) == 0);
}

static void test_lseek(void) {
  const char *path = "/tmp/cpp2rust_fd_io_lseek.tmp";
  int fd = open(path, O_RDWR | O_CREAT | O_TRUNC, 0644);
  assert(fd >= 0);
  assert(write(fd, "hello world", 11) == 11);
  assert(lseek(fd, 0, SEEK_END) == 11);
  assert(lseek(fd, 6, SEEK_SET) == 6);
  char buf[16];
  memset(buf, 0, sizeof(buf));
  assert(read(fd, buf, sizeof(buf)) == 5);
  assert(strcmp(buf, "world") == 0);
  assert(close(fd) == 0);
  assert(unlink(path) == 0);
}

static void test_ftruncate(void) {
  const char *path = "/tmp/cpp2rust_fd_io_trunc.tmp";
  int fd = open(path, O_RDWR | O_CREAT | O_TRUNC, 0644);
  assert(fd >= 0);
  assert(write(fd, "hello world", 11) == 11);
  assert(ftruncate(fd, 5) == 0);
  assert(lseek(fd, 0, SEEK_END) == 5);
  assert(close(fd) == 0);
  assert(unlink(path) == 0);
}

static void test_fstat(void) {
  const char *path = "/tmp/cpp2rust_fd_io_fstat.tmp";
  int fd = open(path, O_WRONLY | O_CREAT | O_TRUNC, 0644);
  assert(fd >= 0);
  assert(write(fd, "hello", 5) == 5);
  struct stat st;
  assert(fstat(fd, &st) == 0);
  assert(st.st_size == 5);
  assert((st.st_mode & S_IFMT) == S_IFREG);
  assert(close(fd) == 0);
  assert(unlink(path) == 0);
}

static void test_isatty(void) {
  const char *path = "/tmp/cpp2rust_fd_io_tty.tmp";
  int fd = open(path, O_RDONLY | O_CREAT, 0644);
  assert(fd >= 0);
  assert(isatty(fd) == 0);
  assert(close(fd) == 0);
  assert(unlink(path) == 0);
}

static void test_tcgetattr(void) {
  const char *path = "/tmp/cpp2rust_fd_io_termios.tmp";
  int fd = open(path, O_RDONLY | O_CREAT, 0644);
  assert(fd >= 0);
  struct termios tio;
  assert(tcgetattr(fd, &tio) == -1);
  assert(close(fd) == 0);
  assert(unlink(path) == 0);
}

static void test_fcntl(void) {
  int fds[2];
  assert(pipe(fds) == 0);
  int flags = fcntl(fds[0], F_GETFL, 0);
  assert(flags >= 0);
  assert((flags & O_NONBLOCK) == 0);
  assert(fcntl(fds[0], F_SETFL, flags | O_NONBLOCK) == 0);
  flags = fcntl(fds[0], F_GETFL, 0);
  assert((flags & O_NONBLOCK) != 0);
  char b;
  assert(read(fds[0], &b, 1) == -1);
  assert(fcntl(fds[0], F_SETFD, FD_CLOEXEC) == 0);
  assert(close(fds[0]) == 0);
  assert(close(fds[1]) == 0);
}

static void test_select(void) {
  int fds[2];
  assert(pipe(fds) == 0);
  fd_set rset;
  FD_ZERO(&rset);
  FD_SET(fds[0], &rset);
  struct timeval tv;
  tv.tv_sec = 0;
  tv.tv_usec = 0;
  assert(select(fds[0] + 1, &rset, NULL, NULL, &tv) == 0);
  assert(!FD_ISSET(fds[0], &rset));
  assert(write(fds[1], "x", 1) == 1);
  FD_ZERO(&rset);
  FD_SET(fds[0], &rset);
  tv.tv_sec = 1;
  tv.tv_usec = 0;
  assert(select(fds[0] + 1, &rset, NULL, NULL, &tv) == 1);
  assert(FD_ISSET(fds[0], &rset));
  assert(close(fds[0]) == 0);
  assert(close(fds[1]) == 0);
}

static void test_poll(void) {
  int fds[2];
  assert(pipe(fds) == 0);
  assert(write(fds[1], "x", 1) == 1);
  struct pollfd pfd[2];
  pfd[0].fd = fds[0];
  pfd[0].events = POLLIN;
  pfd[0].revents = 0;
  pfd[1].fd = -1;
  pfd[1].events = POLLIN;
  pfd[1].revents = 42;
  assert(poll(pfd, 2, 0) == 1);
  assert((pfd[0].revents & POLLIN) != 0);
  assert(pfd[1].revents == 0);
  char ch;
  assert(read(fds[0], &ch, 1) == 1);
  assert(close(fds[0]) == 0);
  assert(close(fds[1]) == 0);
}

int main(void) {
  test_open_read_write();
  test_pipe();
  test_socket_listen();
  test_lseek();
  test_ftruncate();
  test_fstat();
  test_isatty();
  test_tcgetattr();
  test_fcntl();
  test_select();
  test_poll();
  return 0;
}
