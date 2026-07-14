#include <assert.h>

#define END_MARK "ab.cd"

struct label {
  const char *name;
  int (*probe)(void);
  int mask;
};

static int probe_two(void) { return 1; }

#define ENTRY(name, probe, mask) {(name), (probe), (mask)}

static const struct label table[] = {
    ENTRY("first", 0, 1 << 4),
    ENTRY("second", probe_two, 1 << 5),
};

int main() {
  assert(table[0].name[0] == 'f');
  assert(table[0].name[4] == 't');
  assert(table[0].probe == 0);
  assert(table[0].mask == 16);
  assert(table[1].name[0] == 's');
  assert(table[1].probe() == 1);
  assert(table[1].mask == 32);

  const char *tail = &END_MARK[2];
  assert(tail[0] == '.');
  assert(tail[1] == 'c');
  assert(tail[2] == 'd');

  int have = 0;
  void *p = have ? (void *)table[0].name : "";
  assert(((const char *)p)[0] == '\0');
  have = 1;
  p = have ? (void *)table[0].name : "";
  assert(((const char *)p)[0] == 'f');
  return 0;
}
