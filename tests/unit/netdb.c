// no-compile: refcount
#include <assert.h>
#include <netdb.h>
#include <stddef.h>
#include <string.h>
#include <sys/socket.h>

static void test_getaddrinfo(void) {
  struct addrinfo hints;
  memset(&hints, 0, sizeof(hints));
  hints.ai_family = AF_INET;
  hints.ai_socktype = SOCK_STREAM;
  hints.ai_flags = AI_NUMERICHOST;
  struct addrinfo *res = NULL;
  assert(getaddrinfo("127.0.0.1", "80", &hints, &res) == 0);
  assert(res != NULL);
  assert(res->ai_family == AF_INET);
  assert(res->ai_socktype == SOCK_STREAM);
  freeaddrinfo(res);
}

int main(void) {
  test_getaddrinfo();
  return 0;
}
