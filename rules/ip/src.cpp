#include <netinet/in.h>
#include <netinet/tcp.h>

typedef struct sockaddr_in t1;
typedef struct in_addr t2;
typedef struct sockaddr_in6 t3;
typedef struct in6_addr t4;

int f1() {
  return IPPROTO_TCP;
}

int f2() {
  return IPPROTO_UDP;
}

int f3() {
  return IPPROTO_IP;
}

int f4() {
  return IPPROTO_IPV6;
}

#if defined(__linux__)
int f5() {
  return IPPROTO_MPTCP;
}
#endif

int f6() {
  return TCP_NODELAY;
}
