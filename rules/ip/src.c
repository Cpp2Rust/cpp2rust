#ifdef CPP2RUST_USE_INCLUDES
#include <netinet/in.h>
#include <netinet/tcp.h>
#endif

typedef struct sockaddr_in CPP2RUST_TYPE_RULE(1);
typedef struct in_addr CPP2RUST_TYPE_RULE(2);
typedef struct sockaddr_in6 CPP2RUST_TYPE_RULE(3);
typedef struct in6_addr CPP2RUST_TYPE_RULE(4);

int CPP2RUST_EXPR_RULE(1)() { return IPPROTO_TCP; }

int CPP2RUST_EXPR_RULE(2)() { return IPPROTO_UDP; }

int CPP2RUST_EXPR_RULE(3)() { return IPPROTO_IP; }

int CPP2RUST_EXPR_RULE(4)() { return IPPROTO_IPV6; }

#if defined(__linux__)
int CPP2RUST_EXPR_RULE(5)() { return IPPROTO_MPTCP; }
#endif

int CPP2RUST_EXPR_RULE(6)() { return TCP_NODELAY; }
