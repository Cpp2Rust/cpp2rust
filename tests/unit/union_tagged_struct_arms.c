// translation-fail
#include <assert.h>
#include <stdint.h>

typedef enum {
  T_SET = 1,
  T_RANGE_ASCII,
  T_RANGE_NUM,
} VariantKind;

struct Pattern {
  VariantKind kind;
  int index;
  union {
    struct {
      char **elem;
      int64_t size;
      int64_t idx;
    } set;
    struct {
      int min;
      int max;
      int letter;
      unsigned char step;
    } ascii;
    struct {
      int64_t min;
      int64_t max;
      int64_t idx;
      int64_t step;
      int npad;
    } num;
  } v;
};

int main(void) {
  static char *items[] = {"a", "b", "c"};

  struct Pattern p_set;
  p_set.kind = T_SET;
  p_set.index = 0;
  p_set.v.set.elem = items;
  p_set.v.set.size = 3;
  p_set.v.set.idx = 1;
  assert(p_set.v.set.size == 3);
  assert(p_set.v.set.elem[1][0] == 'b');

  struct Pattern p_ascii;
  p_ascii.kind = T_RANGE_ASCII;
  p_ascii.index = 1;
  p_ascii.v.ascii.min = 'a';
  p_ascii.v.ascii.max = 'z';
  p_ascii.v.ascii.letter = 'm';
  p_ascii.v.ascii.step = 1;
  assert(p_ascii.v.ascii.max - p_ascii.v.ascii.min == 25);

  struct Pattern p_num;
  p_num.kind = T_RANGE_NUM;
  p_num.index = 2;
  p_num.v.num.min = 1;
  p_num.v.num.max = 100;
  p_num.v.num.idx = 1;
  p_num.v.num.step = 1;
  p_num.v.num.npad = 3;
  assert(p_num.v.num.max == 100);
  assert(p_num.v.num.npad == 3);

  return 0;
}
