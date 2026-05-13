#include <assert.h>
#include <sys/socket.h>
#include <sys/types.h>

int main() {
  assert(SOCK_STREAM == 1);
  assert(SOCK_DGRAM == 2);
  return 0;
}
