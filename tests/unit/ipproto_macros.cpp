#include <netinet/in.h>

int main() {
  int tcp = IPPROTO_TCP;
  int udp = IPPROTO_UDP;
  int ip = IPPROTO_IP;
  int ip6 = IPPROTO_IPV6;
  int mptcp = IPPROTO_MPTCP;
  return tcp + udp + ip + ip6 + mptcp;
}
