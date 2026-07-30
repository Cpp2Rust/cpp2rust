#include <assert.h>
#include <poll.h>
#include <unistd.h>

int main(void) {
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
  return 0;
}
