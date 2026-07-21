#include <assert.h>
#include <stdio.h>
#include <string.h>

int main() {
  char buf[32];
  assert(snprintf(buf, sizeof(buf), "x=%d y=%u", -3, 7u) == 8);
  assert(strcmp(buf, "x=-3 y=7") == 0);
  assert(snprintf(buf, 4, "%s", "hello") == 5);
  assert(strcmp(buf, "hel") == 0);
  assert(snprintf(buf, sizeof(buf), "%05d|%x|%X", 42, 255, 255) == 11);
  assert(strcmp(buf, "00042|ff|FF") == 0);
  assert(snprintf(buf, sizeof(buf), "%.2f", 3.14159) == 4);
  assert(strcmp(buf, "3.14") == 0);
  assert(snprintf(buf, sizeof(buf), "%-6s|", "ab") == 7);
  assert(strcmp(buf, "ab    |") == 0);
  assert(snprintf(buf, sizeof(buf), "%c%%", 65) == 2);
  assert(strcmp(buf, "A%") == 0);
  assert(snprintf(buf, sizeof(buf), "%+d % d", 5, 5) == 5);
  assert(strcmp(buf, "+5  5") == 0);
  assert(snprintf(buf, sizeof(buf), "%ld %lu %zu", -1L, 1UL, (size_t)9) == 6);
  assert(strcmp(buf, "-1 1 9") == 0);
  assert(snprintf(buf, sizeof(buf), "%e", 1234.5678) == 12);
  assert(strcmp(buf, "1.234568e+03") == 0);
  assert(snprintf(buf, sizeof(buf), "%g", 1234567.0) == 11);
  assert(strcmp(buf, "1.23457e+06") == 0);
  buf[0] = 'Z';
  assert(snprintf(buf, 0, "%d", 123) == 3);
  assert(buf[0] == 'Z');
  char fmt[8];
  fmt[0] = '%';
  fmt[1] = '5';
  fmt[2] = '.';
  fmt[3] = '1';
  fmt[4] = 'f';
  fmt[5] = 0;
  assert(snprintf(buf, sizeof(buf), fmt, 3.26) == 5);
  assert(strcmp(buf, "  3.3") == 0);
  return 0;
}
