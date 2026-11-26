#include <cstdio>

struct Element {
  long id;
  long value;
  ~Element() { printf("%ld:%ld ", id, value); }
};

int main() {
  Element *a = new Element[3]{{1, 10}, {2, 20}, {3, 30}};
  delete[] a;
  printf("\n");

  int n = 1;
  Element *b = new Element[n];
  b[0].id = 4;
  b[0].value = 40;
  delete[] b;
  printf("\n");

  Element *empty = nullptr;
  delete[] empty;
  return 0;
}
