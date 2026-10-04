#ifdef CPP2RUST_USE_INCLUDES
#include <sys/types.h>
#include <ifaddrs.h>
#endif

typedef struct ifaddrs CPP2RUST_TYPE_RULE(1);

int CPP2RUST_EXPR_RULE(1)(struct ifaddrs **ifap) { return getifaddrs(ifap); }

void CPP2RUST_EXPR_RULE(2)(struct ifaddrs *ifa) { return freeifaddrs(ifa); }
