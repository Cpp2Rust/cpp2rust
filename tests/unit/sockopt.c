#include <assert.h>
#include <netinet/in.h>
#include <netinet/tcp.h>
#include <sys/socket.h>
#include <unistd.h>

int main(void) {
  int s = socket(AF_INET, SOCK_STREAM, 0);
  assert(s >= 0);
  int on = 1;
  assert(setsockopt(s, SOL_SOCKET, SO_KEEPALIVE, &on, sizeof(on)) == 0);
  assert(setsockopt(s, IPPROTO_TCP, TCP_NODELAY, &on, sizeof(on)) == 0);
  int err = -1;
  socklen_t len = sizeof(err);
  assert(getsockopt(s, SOL_SOCKET, SO_ERROR, &err, &len) == 0);
  assert(err == 0);
  assert(close(s) == 0);
  return 0;
}
