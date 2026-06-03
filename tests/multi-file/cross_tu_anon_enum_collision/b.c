// anon enum below shares the same declaration line as b.c:enum { ALPHA = 7 }

enum { BETA = 9 };

int b_value(void) {
  int x = 0;
  x |= BETA;
  return x;
}
