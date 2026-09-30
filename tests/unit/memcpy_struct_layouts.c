#include <assert.h>
#include <stdint.h>
#include <string.h>

// No padding.
struct packed {
  int32_t a;
  char b;
  char c;
  int16_t d;
};

// Padding that Rust doesn't need, as it reorders the fields.
struct reordered {
  char a;
  int32_t b;
  char c;
};

// Padding only to align the struct.
struct tail {
  char a;
  double b;
};

struct nested {
  struct tail t;
  char c;
};

// Padding after an array.
struct array {
  char name[3];
  int32_t x;
};

int main(void) {
  unsigned char buf[64];

  struct packed p[2] = {{1, 2, 3, 4}, {5, 6, 7, 8}};
  memcpy(buf, p, sizeof(p));
  assert(buf[sizeof(struct packed) + 4] == 6);
  struct packed p2[2];
  memcpy(p2, buf, sizeof(p2));
  assert(p2[1].a == 5 && p2[1].b == 6 && p2[1].c == 7 && p2[1].d == 8);

  struct reordered r[2] = {{1, 2, 3}, {4, 5, 6}};
  memcpy(buf, r, sizeof(r));
  assert(buf[sizeof(struct reordered) + 8] == 6);
  struct reordered r2[2];
  memcpy(r2, buf, sizeof(r2));
  assert(r2[1].a == 4 && r2[1].b == 5 && r2[1].c == 6);

  struct nested n[2] = {{{1, 2.5}, 3}, {{4, 5.5}, 6}};
  memcpy(buf, n, sizeof(n));
  assert(buf[sizeof(struct nested) + 16] == 6);
  struct nested n2[2];
  memcpy(n2, buf, sizeof(n2));
  assert(n2[1].t.a == 4 && n2[1].t.b == 5.5 && n2[1].c == 6);

  struct array a[2] = {{"ab", 1}, {"cd", 2}};
  memcpy(buf, a, sizeof(a));
  assert(buf[sizeof(struct array) + 1] == 'd');
  struct array a2[2];
  memcpy(a2, buf, sizeof(a2));
  assert(a2[1].name[1] == 'd' && a2[1].name[2] == 0 && a2[1].x == 2);

  return 0;
}
