#include <arpa/inet.h>
#include <assert.h>
#include <netinet/in.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

static void check_name(struct sockaddr *addr, socklen_t expected) {
  int receiver = socket(addr->sa_family, SOCK_DGRAM, 0);
  int sender = socket(addr->sa_family, SOCK_DGRAM, 0);
  assert(receiver >= 0);
  assert(sender >= 0);

  struct sockaddr_storage local;
  socklen_t len = sizeof(local);
  assert(getsockname(receiver, (struct sockaddr *)&local, &len) == 0);
  assert(len == expected);
  len = 0;
  assert(getsockname(receiver, (struct sockaddr *)&local, &len) == 0);
  assert(len == expected);

  assert(bind(receiver, addr, expected) == 0);
  len = sizeof(local);
  assert(getsockname(receiver, (struct sockaddr *)&local, &len) == 0);
  assert(len == expected);
  assert(connect(sender, (struct sockaddr *)&local, len) == 0);

  struct sockaddr_storage peer;
  len = sizeof(peer);
  assert(getpeername(sender, (struct sockaddr *)&peer, &len) == 0);
  assert(len == expected);
  assert(memcmp(&local, &peer, expected) == 0);

  char sent = 'x', received = 0;
  assert(send(sender, &sent, 1, 0) == 1);
  struct sockaddr_storage from;
  len = sizeof(from);
  assert(recvfrom(receiver, &received, 1, 0, (struct sockaddr *)&from, &len) ==
         1);
  assert(len == expected);
  assert(received == sent);
  len = sizeof(local);
  assert(getsockname(sender, (struct sockaddr *)&local, &len) == 0);
  assert(len == expected);
  assert(memcmp(&local, &from, expected) == 0);

  received = 0;
  assert(send(sender, &sent, 1, 0) == 1);
  assert(recvfrom(receiver, &received, 1, 0, NULL, NULL) == 1);
  assert(received == sent);
  received = 0;
  len = 77;
  assert(send(sender, &sent, 1, 0) == 1);
  assert(recvfrom(receiver, &received, 1, 0, NULL, &len) == 1);
  assert(len == 77);
  assert(received == sent);

  assert(close(sender) == 0);
  assert(close(receiver) == 0);
}

int main(void) {
  struct sockaddr_in in;
  memset(&in, 0, sizeof(in));
  in.sin_family = AF_INET;
  in.sin_addr.s_addr = htonl(0x7f000001);
  check_name((struct sockaddr *)&in, sizeof(in));

  struct sockaddr_in6 in6;
  int fd = socket(AF_INET6, SOCK_DGRAM, 0);
  assert(fd >= 0);
  socklen_t len = sizeof(in6);
  assert(getsockname(fd, (struct sockaddr *)&in6, &len) == 0);
  assert(len == sizeof(struct sockaddr_in6));
  assert(close(fd) == 0);
  return 0;
}
