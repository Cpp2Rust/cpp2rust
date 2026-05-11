#include <netinet/in.h>

int f1() {
  return IPPROTO_TCP;
}

int f2() {
  return IPPROTO_UDP;
}

int f3() {
  return IPPROTO_IP;
}
