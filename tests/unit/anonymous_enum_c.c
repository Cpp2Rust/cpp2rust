#include <assert.h>

enum {
  FIRST_A,
  FIRST_B,
};

struct S {
  int a;

  enum {
    SECOND_A,
    SECOND_B,
  };
};

typedef enum {
  TD_A,
  TD_B,
} TdEnum;

struct WithAnonField {
  int a;
  enum {
    FIELD_A,
    FIELD_B,
  } field;
};

int main() {
  enum {
    THIRD_A,
    THIRD_B,
  };

  assert(FIRST_A == 0);
  assert(FIRST_B == 1);

  assert(SECOND_A == 0);
  assert(SECOND_B == 1);

  assert(THIRD_A == 0);
  assert(THIRD_B == 1);

  TdEnum td = TD_A;
  assert(td == TD_A);
  td = TD_B;
  assert(td == TD_B);

  struct WithAnonField w;
  w.field = FIELD_A;
  assert(w.field == FIELD_A);
  w.field = FIELD_B;
  assert(w.field == FIELD_B);

  return 0;
};
