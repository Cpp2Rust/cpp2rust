#include <cassert>

int main() {
  int i1 = 42;
  int *ptr1 = &i1;
  char *ptr2 = reinterpret_cast<char *>(&i1);

  void *vptr1 = ptr1;
  void *vptr2 = ptr2;
  assert(vptr1 == vptr2);
  return 0;
}
