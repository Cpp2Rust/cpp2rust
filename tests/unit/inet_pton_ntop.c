#include <arpa/inet.h>
#include <assert.h>
#include <string.h>

int main(void) {
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
