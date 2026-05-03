#include <assert.h>
#include <stdbool.h>
#include <stddef.h>

int cmp_eq(int rc) { return rc == -1; }
int cmp_or_ptr(const char *p, const char *q) { return p || q; }
int both_null(const char *s1, const char *s2) {
  return (s1 == nullptr) && (s2 == nullptr);
}

int main() {
  int a = 0;
  int b;

  if (b = a) {
  }
  while ((b = a) != 0) {
  }
  if (a) {
  }
  if (a == b) {
  }
  if (a < b) {
  }

  assert(a == b);
  assert(!(a = b));

  bool c;
  c = a = b;
  c = (b = a) != 0;
  c = a;
  c = a == b;
  c = a < b;

  int x = 5, y = 5;
  int eq = x == y;
  int lt = x < y;
  int neq = x != y;
  assert(eq == 1);
  assert(lt == 0);
  assert(neq == 0);

  const char *p1 = "hi";
  const char *p2 = nullptr;
  int either = p1 || p2;
  int both = p1 && p2;
  assert(either == 1);
  assert(both == 0);

  assert(cmp_eq(-1) == 1);
  assert(cmp_eq(0) == 0);
  assert(cmp_or_ptr(p1, p2) == 1);
  assert(cmp_or_ptr(nullptr, nullptr) == 0);
  assert(both_null(nullptr, nullptr) == 1);
  assert(both_null(p1, nullptr) == 0);

  return 0;
}
