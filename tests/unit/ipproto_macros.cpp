#include <cassert>
#include <netinet/in.h>

int main() {
  int tcp = IPPROTO_TCP;
  int udp = IPPROTO_UDP;
  int ip = IPPROTO_IP;
  int ip6 = IPPROTO_IPV6;
  assert(tcp + udp + ip + ip6 == 64);
  return 0;
}
