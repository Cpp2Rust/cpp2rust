#include <assert.h>
#include <cstdio>

static int sum_bytes(const char *buf, unsigned len) {
  int sum = 0;
  for (unsigned i = 0; i < len; i++) {
    sum += (unsigned char)buf[i];
  }
  return sum;
}

static const char *g_packet = "\x01\x00";

int main() {
  int a = sum_bytes("\x01\x00", 2);
  int b = sum_bytes(g_packet, 2);

  assert(a == b);
  assert(a == 1);

  int c = "\r\n.\r\n"[0] + "\r\n.\r\n"[3];
  assert(c == '\r' + '\r');

  int idx = 1;
  int d = "abcd"[idx];
  assert(d == 'b');

  return 0;
}
