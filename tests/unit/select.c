#include <assert.h>
#include <string.h>
#include <sys/select.h>
#include <unistd.h>

int main(void) {
  int fds[2];
  assert(pipe(fds) == 0);
  fd_set rset;
  FD_ZERO(&rset);
  FD_SET(fds[0], &rset);
  struct timeval tv;
  memset(&tv, 0, sizeof(tv));
  tv.tv_sec = 0;
  assert(select(fds[0] + 1, &rset, NULL, NULL, &tv) == 0);
  assert(!FD_ISSET(fds[0], &rset));
  assert(write(fds[1], "x", 1) == 1);
  FD_ZERO(&rset);
  FD_SET(fds[0], &rset);
  tv.tv_sec = 1;
  assert(select(fds[0] + 1, &rset, NULL, NULL, &tv) == 1);
  assert(FD_ISSET(fds[0], &rset));
  assert(close(fds[0]) == 0);
  assert(close(fds[1]) == 0);
  return 0;
}
