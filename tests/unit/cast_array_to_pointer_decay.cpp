#include <cassert>

int deref(int *p) { return *p; }
int strlen(char *s) {
  int c = 0;
  while (*s++)
    ++c;
  return c;
}
struct Row {
  int a[3];
};
int main() {
  int a[2] = {1, 2};
  char s[4] = {'a', 'b', 'c', '\0'};
  assert(deref(a) + strlen(s) == 4);

  // Pointers to arrays, which point to the whole array.
  int m[2][3] = {{1, 2, 3}, {4, 5, 6}};
  Row r = {{7, 8, 9}};
  int (*row)[3] = &m[1];
  int (*field)[3] = &r.a;
  int (*whole)[3] = &m[0];
  assert((*row)[2] == 6 && (*field)[1] == 8 && (*whole)[0] == 1);
  assert((int *)row == m[1] && *(int *)field == 7 && ((int *)&r.a)[2] == 9);
  void *v = &r.a;
  assert(v == (void *)r.a && v != (void *)row);
  return 0;
}
