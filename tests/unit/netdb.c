#include <assert.h>
#include <netdb.h>
#include <netinet/in.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/types.h>

static void test_ipv4_literal(void) {
  struct addrinfo hints = {0};
  hints.ai_family = AF_INET;
  hints.ai_socktype = SOCK_STREAM;
  struct addrinfo *res = NULL;
  assert(getaddrinfo("127.0.0.1", "8080", &hints, &res) == 0);
  assert(res != NULL);
  assert(res->ai_family == AF_INET);
  assert(res->ai_socktype == SOCK_STREAM);
  assert(res->ai_addrlen == sizeof(struct sockaddr_in));
  assert(res->ai_addr != NULL);
  struct sockaddr_in *sin = (struct sockaddr_in *)res->ai_addr;
  assert(sin->sin_family == AF_INET);
  unsigned char port_be[2] = {8080 / 256, 8080 % 256};
  assert(memcmp(&sin->sin_port, port_be, 2) == 0);
  unsigned char addr_be[4] = {127, 0, 0, 1};
  assert(memcmp(&sin->sin_addr, addr_be, 4) == 0);
  freeaddrinfo(res);
}

static void test_ipv6_literal(void) {
  struct addrinfo hints = {0};
  hints.ai_family = AF_INET6;
  hints.ai_socktype = SOCK_STREAM;
  struct addrinfo *res = NULL;
  assert(getaddrinfo("::1", "443", &hints, &res) == 0);
  assert(res != NULL);
  assert(res->ai_family == AF_INET6);
  assert(res->ai_addrlen == sizeof(struct sockaddr_in6));
  assert(res->ai_addr != NULL);
  struct sockaddr_in6 *sin6 = (struct sockaddr_in6 *)res->ai_addr;
  assert(sin6->sin6_family == AF_INET6);
  unsigned char port_be[2] = {443 / 256, 443 % 256};
  assert(memcmp(&sin6->sin6_port, port_be, 2) == 0);
  unsigned char addr_be[16] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1};
  assert(memcmp(&sin6->sin6_addr, addr_be, 16) == 0);
  freeaddrinfo(res);
}

static void test_family_mismatch(void) {
  struct addrinfo hints = {0};
  hints.ai_family = AF_INET6;
  hints.ai_socktype = SOCK_STREAM;
  struct addrinfo *res = NULL;
  assert(getaddrinfo("127.0.0.1", "80", &hints, &res) != 0);
}

static void test_null_hints(void) {
  struct addrinfo *res = NULL;
  assert(getaddrinfo("127.0.0.1", "80", NULL, &res) == 0);
  assert(res != NULL);
  assert(res->ai_family == AF_INET);
  struct sockaddr_in *sin = (struct sockaddr_in *)res->ai_addr;
  unsigned char addr_be[4] = {127, 0, 0, 1};
  assert(memcmp(&sin->sin_addr, addr_be, 4) == 0);
  freeaddrinfo(res);
}

int main(void) {
  test_ipv4_literal();
  test_ipv6_literal();
  test_family_mismatch();
  test_null_hints();
  return 0;
}
