#include <ifaddrs.h>
#include <sys/types.h>

typedef struct ifaddrs t1;

int f1(struct ifaddrs **ifap) { return getifaddrs(ifap); }

void f2(struct ifaddrs *ifa) { return freeifaddrs(ifa); }
