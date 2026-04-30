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

  assert(FIRST_A != FIRST_B);
  assert(SECOND_A != SECOND_B);
  assert(THIRD_A != THIRD_B);

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
