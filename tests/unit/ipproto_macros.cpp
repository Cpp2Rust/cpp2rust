#include <netinet/in.h>

int main() {
  int tcp = IPPROTO_TCP;
  int udp = IPPROTO_UDP;
  int ip = IPPROTO_IP;
  return tcp + udp + ip;
}
