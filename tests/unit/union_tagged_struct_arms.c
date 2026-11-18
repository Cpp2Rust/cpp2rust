#include <assert.h>
#include <stdint.h>

typedef enum {
  C_LIST = 1,
  C_LETTERS,
  C_INTEGERS,
} Choice;

struct Branch {
  Choice choice;
  int index;
  union {
    struct {
      char **items;
      int64_t count;
      int64_t cursor;
    } list;
    struct {
      int lo;
      int hi;
      int curr;
      unsigned char step;
    } letters;
    struct {
      int64_t lo;
      int64_t hi;
      int64_t curr;
      int64_t step;
      int width;
    } integers;
  } v;
};

int main(void) {
  static char *items[] = {"a", "b", "c"};

  struct Branch p_list;
  p_list.choice = C_LIST;
  p_list.index = 0;
  p_list.v.list.items = items;
  p_list.v.list.count = 3;
  p_list.v.list.cursor = 1;
  assert(p_list.v.list.count == 3);
  assert(p_list.v.list.items[1][0] == 'b');

  struct Branch p_letters;
  p_letters.choice = C_LETTERS;
  p_letters.index = 1;
  p_letters.v.letters.lo = 'a';
  p_letters.v.letters.hi = 'z';
  p_letters.v.letters.curr = 'm';
  p_letters.v.letters.step = 1;
  assert(p_letters.v.letters.hi - p_letters.v.letters.lo == 25);

  struct Branch p_integers;
  p_integers.choice = C_INTEGERS;
  p_integers.index = 2;
  p_integers.v.integers.lo = 1;
  p_integers.v.integers.hi = 100;
  p_integers.v.integers.curr = 1;
  p_integers.v.integers.step = 1;
  p_integers.v.integers.width = 3;
  assert(p_integers.v.integers.hi == 100);
  assert(p_integers.v.integers.width == 3);

  return 0;
}
