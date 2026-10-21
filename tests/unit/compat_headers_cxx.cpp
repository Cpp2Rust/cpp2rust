#include <arpa/inet.h>
#include <assert.h>
#include <errno.h>
#include <sys/types.h>
#include <sys/select.h>

int main() {
  fd_set set;
  FD_ZERO(&set);
  FD_SET(3, &set);
  assert(FD_ISSET(3, &set));
  errno = 42;
  assert(errno == 42);
  assert(ntohl(htonl(7)) == 7);
  return 0;
}
