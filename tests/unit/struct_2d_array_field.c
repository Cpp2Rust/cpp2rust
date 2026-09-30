#include <assert.h>
#include <stddef.h>
#include <string.h>

struct table {
  char rows[3][10];
  size_t count;
};

static const struct table T1 = {{"alpha"}, 1};
static const struct table T2 = {{"alpha", "beta"}, 2};

int main(void) {
  assert(T1.count == 1);
  assert(strcmp(T1.rows[0], "alpha") == 0);
  assert(T1.rows[1][0] == '\0');

  assert(T2.count == 2);
  assert(strcmp(T2.rows[0], "alpha") == 0);
  assert(strcmp(T2.rows[1], "beta") == 0);
  assert(T2.rows[2][0] == '\0');

  struct table local = {{"one", "two", "three"}, 3};
  assert(strcmp(local.rows[2], "three") == 0);
  local.rows[1][0] = 'T';
  assert(strcmp(local.rows[1], "Two") == 0);
  assert(strcmp(local.rows[0], "one") == 0);

  const char *p = local.rows[2];
  assert(p[0] == 't');
  assert(local.count == 3);
  return 0;
}
