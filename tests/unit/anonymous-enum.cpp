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

int main() {
  enum {
    THIRD_A,
    THIRD_B,
  };

  assert(FIRST_A == 0);
  assert(FIRST_B == 1);

  assert(S::SECOND_A == 0);
  assert(S::SECOND_B == 1);

  assert(THIRD_A == 0);
  assert(THIRD_B == 1);

  return 0;
};
