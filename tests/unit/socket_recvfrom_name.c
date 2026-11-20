#include <assert.h>
#include <stddef.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>

static void check_from(int receiver, int sender,
                       struct sockaddr_storage *expected,
                       socklen_t expected_len) {
  char sent = 'x', received = 0;
  assert(send(sender, &sent, 1, 0) == 1);
  struct sockaddr_storage from;
  memset(&from, 0xa5, sizeof(from));
  socklen_t len = sizeof(from);
  assert(recvfrom(receiver, &received, 1, 0, (struct sockaddr *)&from, &len) ==
         1);
  assert(len == expected_len);
  assert(received == sent);
  assert(memcmp(&from, expected, len) == 0);
  for (unsigned i = len; i < sizeof(from); ++i)
    assert(((unsigned char *)&from)[i] == 0xa5);
}

int main(void) {
  int receiver = socket(AF_UNIX, SOCK_DGRAM, 0);
  int sender = socket(AF_UNIX, SOCK_DGRAM, 0);
  assert(receiver >= 0);
  assert(sender >= 0);

  struct sockaddr_storage addr;
  socklen_t len = sizeof(addr);
  assert(getsockname(receiver, (struct sockaddr *)&addr, &len) == 0);
  struct sockaddr_un *un = (struct sockaddr_un *)&addr;
  memcpy(un->sun_path, "recvfrom-receiver", sizeof("recvfrom-receiver"));
  len = offsetof(struct sockaddr_un, sun_path) + strlen(un->sun_path) + 1;
  assert(bind(receiver, (struct sockaddr *)&addr, len) == 0);
  assert(connect(sender, (struct sockaddr *)&addr, len) == 0);
  char received = 0;
  assert(send(sender, "x", 1, 0) == 1);
  struct sockaddr_storage from;
  socklen_t from_len = sizeof(from);
  assert(recvfrom(receiver, &received, 1, 0, (struct sockaddr *)&from,
                  &from_len) == 1);
  assert(received == 'x');
  assert(send(sender, "x", 1, 0) == 1);
  assert(recvfrom(receiver, &received, 1, 0, NULL, NULL) == 1);
  assert(received == 'x');
  received = 0;
  len = 77;
  assert(send(sender, "x", 1, 0) == 1);
  assert(recvfrom(receiver, &received, 1, 0, NULL, &len) == 1);
  assert(len == 77);
  assert(received == 'x');

  memcpy(un->sun_path, "recvfrom-sender", sizeof("recvfrom-sender"));
  len = offsetof(struct sockaddr_un, sun_path) + strlen(un->sun_path) + 1;
  assert(bind(sender, (struct sockaddr *)&addr, len) == 0);
  len = sizeof(addr);
  assert(getsockname(sender, (struct sockaddr *)&addr, &len) == 0);
  check_from(receiver, sender, &addr, len);
  received = 0;
  assert(send(sender, "x", 1, 0) == 1);
  assert(recvfrom(receiver, &received, 1, 0, NULL, NULL) == 1);
  assert(received == 'x');
  received = 0;
  len = 77;
  assert(send(sender, "x", 1, 0) == 1);
  assert(recvfrom(receiver, &received, 1, 0, NULL, &len) == 1);
  assert(len == 77);
  assert(received == 'x');
  assert(close(sender) == 0);
  assert(close(receiver) == 0);
  assert(unlink("recvfrom-sender") == 0);
  assert(unlink("recvfrom-receiver") == 0);

  return 0;
}
