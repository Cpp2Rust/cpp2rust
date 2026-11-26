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

  return 0;
}
