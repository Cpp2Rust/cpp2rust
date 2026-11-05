#include <cstdio>

static void put(FILE &out, const char *s) { fputs(s, &out); }

int main() {
  FILE *fp = stdout;
  put(*fp, "value\n");
  return 0;
}
