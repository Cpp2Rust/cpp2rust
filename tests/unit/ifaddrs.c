#include <assert.h>
#include <ifaddrs.h>
#include <net/if.h>
#include <netinet/in.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/types.h>

int main(void) {
  struct ifaddrs *list = NULL;
  assert(getifaddrs(&list) == 0);
  assert(list != NULL);
  int found_loopback = 0;
  struct ifaddrs *ifa;
  for (ifa = list; ifa != NULL; ifa = ifa->ifa_next) {
    assert(ifa->ifa_name != NULL);
    if (ifa->ifa_addr == NULL) {
      continue;
    }
    if (ifa->ifa_addr->sa_family != AF_INET) {
      continue;
    }
    struct sockaddr_in *sin = (struct sockaddr_in *)ifa->ifa_addr;
    unsigned char lo_be[4] = {127, 0, 0, 1};
    if (memcmp(&sin->sin_addr, lo_be, 4) == 0) {
      found_loopback = 1;
      assert(ifa->ifa_flags != 0);
      assert(ifa->ifa_netmask != NULL);
      struct sockaddr_in *mask = (struct sockaddr_in *)ifa->ifa_netmask;
      unsigned char mask_be[4] = {255, 0, 0, 0};
      assert(memcmp(&mask->sin_addr, mask_be, 4) == 0);
      assert(if_nametoindex(ifa->ifa_name) > 0);
    }
  }
  assert(found_loopback);
  freeifaddrs(list);

  assert(if_nametoindex("cpp2rust_no_such_if") == 0);
  return 0;
}
