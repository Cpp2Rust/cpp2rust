#include <assert.h>

enum Color { RED, GREEN, BLUE };

enum Option {
  OPT_NONE = 0,
  OPT_A = 10,
  OPT_B = 20,
  OPT_C = 30,
};

typedef enum {
  TAG_ZERO = 0,
  TAG_ONE = 1,
  TAG_TWO = 2,
} Tag;

struct Entry {
  const char *name;
  enum Color color;
  enum Option opt;
};

static enum Color global_color = GREEN;
static enum Option global_opt = OPT_B;
static Tag global_tag = TAG_TWO;

static struct Entry entries[3] = {
    {"first", RED, OPT_NONE},
    {"second", GREEN, OPT_A},
    {"third", BLUE, OPT_C},
};

int as_int(enum Color c) { return c; }

int classify_option(int option) {
  switch (option) {
  case OPT_NONE:
    return -1;
  case OPT_A:
    return 1;
  case OPT_B:
    return 2;
  case OPT_C:
    return 3;
  default:
    return 0;
  }
}

enum Color make_color(int n) { return (enum Color)n; }

int main() {
  enum Color c = RED;

  assert(c == RED);
  assert(c == 0);
  assert(c != 1);

  if (c == GREEN) {
    return 1;
  }

  switch (c) {
  case 0:
    break;
  case 1:
    return 1;
  case 2:
    return 2;
  default:
    return 99;
  }

  int x = c;
  assert(x == 0);

  int y = c + 1;
  assert(y == 1);

  c = (enum Color)2;
  assert(c == BLUE);
  assert(c == 2);

  c = make_color(1);
  assert(c == GREEN);

  enum Color cmp = (enum Color)(c + 1);
  assert(cmp == BLUE);

  enum Option o = OPT_A;
  assert(o == OPT_A);
  assert(o == 10);

  int oi = o;
  assert(oi == 10);

  o = (enum Option)20;
  assert(o == OPT_B);

  int rc = classify_option(o);
  assert(rc == 2);

  rc = classify_option(20);
  assert(rc == 2);

  rc = classify_option(OPT_C);
  assert(rc == 3);

  Tag t = TAG_ONE;
  assert(t == 1);
  assert(t == TAG_ONE);

  int ti = t;
  assert(ti == 1);

  t = (Tag)2;
  assert(t == TAG_TWO);

  switch (t) {
  case TAG_ZERO:
    return 90;
  case 1:
    return 91;
  case 2:
    break;
  }

  int extra = (int)RED + (int)GREEN + (int)BLUE;
  assert(extra == 0 + 1 + 2);

  assert(global_color == GREEN);
  assert(global_opt == OPT_B);
  assert(global_tag == TAG_TWO);

  assert(entries[0].color == RED);
  assert(entries[0].opt == OPT_NONE);
  assert(entries[1].color == GREEN);
  assert(entries[1].opt == OPT_A);
  assert(entries[2].color == BLUE);
  assert(entries[2].opt == OPT_C);

  return 0;
}
