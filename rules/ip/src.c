#include <netinet/in.h>
#include <netinet/tcp.h>

typedef struct sockaddr_in t1;
typedef struct in_addr t2;
typedef struct sockaddr_in6 t3;
typedef struct in6_addr t4;
typedef in_addr_t t5;
typedef in_port_t t6;

int f1(void) { return IPPROTO_TCP; }

int f2(void) { return IPPROTO_UDP; }

int f3(void) { return IPPROTO_IP; }

int f4(void) { return IPPROTO_IPV6; }

#if defined(__linux__)
int f5(void) { return IPPROTO_MPTCP; }
#endif

int f6(void) { return TCP_NODELAY; }

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-stack-address"
uint8_t *f7(struct in6_addr a0) { return a0.s6_addr; }
#pragma clang diagnostic pop
