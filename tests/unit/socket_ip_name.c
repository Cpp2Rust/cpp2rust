#include <assert.h>
#include <netinet/in.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

static void check_buf(int fd, unsigned char *buf, unsigned size, socklen_t cap,
                      const void *full, socklen_t actual) {
  memset(buf, 0xa5, size);
  socklen_t len = cap;
  assert(getsockname(fd, (struct sockaddr *)buf, &len) == 0);
  unsigned copied = cap < actual ? cap : actual;
  assert(memcmp(buf, full, copied) == 0);
  for (unsigned j = copied; j < size; ++j)
    assert(buf[j] == 0xa5);
}

static void check_name(int family) {
  int fd = socket(family, SOCK_DGRAM, 0);
  assert(fd >= 0);
  struct sockaddr_storage full;
  memset(&full, 0xa5, sizeof(full));
  socklen_t len = sizeof(full);
  assert(getsockname(fd, (struct sockaddr *)&full, &len) == 0);
  assert(full.ss_family == family);
  socklen_t actual = family == AF_INET ? sizeof(struct sockaddr_in)
                                       : sizeof(struct sockaddr_in6);

  socklen_t capacities[] = {0, 1, 2, actual - 1, actual, actual + 1};
  for (unsigned i = 0; i < sizeof(capacities) / sizeof(capacities[0]); ++i) {
    unsigned char out[sizeof(struct sockaddr_storage) + 8];
    check_buf(fd, out, sizeof(out), capacities[i], &full, actual);

    unsigned size = capacities[i] ? capacities[i] : 1;
    unsigned char *small = malloc(size);
    assert(small);
    check_buf(fd, small, size, capacities[i], &full, actual);
    free(small);
  }
  assert(close(fd) == 0);
}

int main(void) {
  check_name(AF_INET);
  check_name(AF_INET6);
  return 0;
}
