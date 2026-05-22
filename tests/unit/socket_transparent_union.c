// no-compile: refcount
#define _GNU_SOURCE
#include <assert.h>
#include <errno.h>
#include <sys/socket.h>
#include <sys/types.h>

int main(void) {
  int fd = 0;
  struct sockaddr_storage ssloc;
  socklen_t slen = sizeof(ssloc);
  assert(getsockname(fd, (struct sockaddr *)&ssloc, &slen) == -1);
  assert(errno == ENOTSOCK);
  return 0;
}
