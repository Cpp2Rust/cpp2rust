#define _GNU_SOURCE
#include <assert.h>
#include <netinet/in.h>
#include <sys/socket.h>
#include <sys/types.h>

int main(void) {
  int fd = 0;

  struct sockaddr_storage ssloc;
  socklen_t slen = sizeof(ssloc);
  assert(getsockname(fd, (struct sockaddr *)&ssloc, &slen) == -1);

  struct sockaddr_in sin;
  socklen_t inlen = sizeof(sin);
  assert(getsockname(fd, (struct sockaddr *)&sin, &inlen) == -1);

  return 0;
}
