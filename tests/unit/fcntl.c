#include <assert.h>
#include <fcntl.h>
#include <unistd.h>

int main(void) {
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
  assert((fcntl(fds[0], F_GETFD, 0) & FD_CLOEXEC) == 0);
  assert(fcntl(fds[0], F_SETFD, FD_CLOEXEC) == 0);
  assert((fcntl(fds[0], F_GETFD, 0) & FD_CLOEXEC) != 0);
  assert(close(fds[0]) == 0);
  assert(close(fds[1]) == 0);
  return 0;
}
