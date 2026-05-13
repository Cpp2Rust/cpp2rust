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
  p.revents = 2;
  assert(p.fd == -1);
  assert(p.events == 0);
  assert(p.revents == 2);

  struct in_addr ia;
  ia.s_addr = 1;
  assert(ia.s_addr == 1);

  struct tm t;
  t.tm_year = 124;
  t.tm_mon = 5;
  t.tm_mday = 15;
  assert(t.tm_year == 124);
  assert(t.tm_mon == 5);
  assert(t.tm_mday == 15);

  struct stat st;
  st.st_size = 1024;
  assert(st.st_size == 1024);
  return 0;
}
