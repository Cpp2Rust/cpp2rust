#include <assert.h>

const char *get_greeting(void) { return "hello"; }
const char *get_empty(void) { return ""; }

const char *get_branch(int x) {
  if (x > 0) {
    return "positive";
  }
  return "non-positive";
}

int main(void) {
  const char *a = get_greeting();
  assert(a[0] == 'h');
  assert(a[4] == 'o');
  assert(a[5] == '\0');

  const char *b = get_empty();
  assert(b[0] == '\0');

  const char *c = get_branch(1);
  assert(c[0] == 'p');
  assert(c[7] == 'e');
  assert(c[8] == '\0');

  const char *d = get_branch(-1);
  assert(d[0] == 'n');
  assert(d[11] == 'e');
  assert(d[12] == '\0');
  return 0;
}
