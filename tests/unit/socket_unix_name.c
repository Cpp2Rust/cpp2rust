#include <assert.h>
#include <stddef.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>

static void check_name(int fd, const char *path) {
  struct sockaddr_storage full;
  memset(&full, 0xa5, sizeof(full));
  socklen_t actual = sizeof(full);
  assert(getsockname(fd, (struct sockaddr *)&full, &actual) == 0);
  assert(full.ss_family == AF_UNIX);
  assert(actual >= offsetof(struct sockaddr_un, sun_path));
  assert(actual < sizeof(full));
  if (path)
    assert(memcmp(((struct sockaddr_un *)&full)->sun_path, path,
                  strlen(path)) == 0);

  socklen_t capacities[] = {0, 1, 2, actual - 1, actual, actual + 1};
  for (unsigned i = 0; i < sizeof(capacities) / sizeof(capacities[0]); ++i) {
    unsigned char out[sizeof(struct sockaddr_storage) + 8];
    memset(out, 0xa5, sizeof(out));
    socklen_t len = capacities[i];
    assert(getsockname(fd, (struct sockaddr *)out, &len) == 0);
    assert(len == actual);
    unsigned copied = capacities[i] < actual ? capacities[i] : actual;
    assert(memcmp(out, &full, copied) == 0);
    for (unsigned j = copied; j < sizeof(out); ++j)
      assert(out[j] == 0xa5);
  }
}

int main(void) {
  int fd = socket(AF_UNIX, SOCK_DGRAM, 0);
  assert(fd >= 0);
  check_name(fd, NULL);

  struct sockaddr_storage addr;
  memset(&addr, 0, sizeof(addr));
  socklen_t len = sizeof(addr);
  assert(getsockname(fd, (struct sockaddr *)&addr, &len) == 0);
  struct sockaddr_un *un = (struct sockaddr_un *)&addr;
  memcpy(un->sun_path, "socket-unix-name", sizeof("socket-unix-name"));
  assert(bind(fd, (struct sockaddr *)&addr,
              offsetof(struct sockaddr_un, sun_path) + strlen(un->sun_path)) ==
         0);
  check_name(fd, un->sun_path);
  assert(close(fd) == 0);
  assert(unlink(un->sun_path) == 0);
  return 0;
}
