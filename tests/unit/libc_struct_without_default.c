// no-compile: refcount
#include <assert.h>
#include <netinet/in.h>
#include <poll.h>
#include <sys/stat.h>
#include <time.h>

int main() {
  struct pollfd p;
  p.fd = -1;
  p.events = 0;
  p.revents = 0;

  struct in_addr ia;
  ia.s_addr = 0;
  assert(ia.s_addr == 0);

  struct tm t;
  t.tm_year = 124;
  t.tm_mon = 5;
  t.tm_mday = 15;
  assert(t.tm_year == 124);
  assert(t.tm_mon == 5);
  assert(t.tm_mday == 15);

  struct sockaddr_in sa;
  sa.sin_family = AF_INET;
  sa.sin_port = 8080;
  assert(sa.sin_family == AF_INET);
  assert(sa.sin_port == 8080);

  struct stat st;
  st.st_size = 1024;
  assert(st.st_size == 1024);
  assert(p.fd == -1);
  assert(p.events == 0);
  return 0;
}
