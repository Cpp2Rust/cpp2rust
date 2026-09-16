#include <cassert>

struct XX {
  int x;
};

int main() {
  XX obj, *ptr = &obj;
  ptr->x = 2;
  bool c = false;
  int r = c ? obj.x : ptr->x;
  int *p = &obj.x;
  assert(*p + r == 4);
  return 0;
}
