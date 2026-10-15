#include <arpa/inet.h>
#include <assert.h>
#include <string.h>

static void test_s6_addr_access(void) {
  const unsigned char loopback[16] = {0, 0, 0, 0, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 1};
  struct in6_addr addr;
  struct in6_addr *ap = &addr;
  memcpy(addr.s6_addr, loopback, 16);
  assert(memcmp(ap->s6_addr, loopback, 16) == 0);
  addr.s6_addr[0] = 0x20;
  ap->s6_addr[1] = 0x01;
  assert(addr.s6_addr[0] == 0x20 && ap->s6_addr[1] == 0x01);
  addr.s6_addr[15] += 4;
  ap->s6_addr[15]++;
  assert(addr.s6_addr[15] == 6);
  unsigned char *bytes = addr.s6_addr;
  bytes[2] = 7;
  assert(ap->s6_addr[2] == 7);
  unsigned char *fourth = &ap->s6_addr[3];
  *fourth = 8;
  assert(addr.s6_addr[3] == 8);
  memcpy(ap->s6_addr, loopback, 16);
  assert(memcmp(addr.s6_addr, loopback, 16) == 0);
}

int main(void) {
  test_s6_addr_access();
  unsigned char buf[16];
  assert(inet_pton(AF_INET, "1.2.3.4", buf) == 1);
  assert(buf[0] == 1 && buf[1] == 2 && buf[2] == 3 && buf[3] == 4);
  assert(inet_pton(AF_INET, "999.1.1.1", buf) == 0);
  assert(inet_pton(AF_INET, "not an ip", buf) == 0);
  assert(inet_pton(AF_INET6, "::1", buf) == 1);
  assert(buf[0] == 0 && buf[15] == 1);
  assert(inet_pton(AF_INET6, "2001:db8::5", buf) == 1);
  assert(buf[0] == 0x20 && buf[1] == 0x01 && buf[15] == 5);

  char text[64];
  unsigned char four[4] = {10, 0, 0, 1};
  assert(strcmp(inet_ntop(AF_INET, four, text, sizeof(text)), "10.0.0.1") == 0);
  unsigned char sixteen[16] = {0};
  sixteen[15] = 1;
  assert(strcmp(inet_ntop(AF_INET6, sixteen, text, sizeof(text)), "::1") == 0);
  assert(inet_ntop(AF_INET, four, text, 4) == 0);
  return 0;
}
