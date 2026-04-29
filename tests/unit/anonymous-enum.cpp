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
  return 0;
};
