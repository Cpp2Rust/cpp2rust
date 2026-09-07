#include <assert.h>

int main() {
  int x = 1;
  {
    int x = 2;
    assert(x == 2);
    {
      int x = 3;
      assert(x == 3);
    }
    assert(x == 2);
  }
  assert(x == 1);

  int sum = 0;
  for (int i = 0; i < 3; i++) {
    int y = i;
    {
      int y = 10;
      sum += y;
    }
    sum += y;
  }
  assert(sum == 33);

  if (x == 1) {
    int x = 5;
    {
      int x = 6;
      assert(x == 6);
    }
    assert(x == 5);
  }
  assert(x == 1);
  return 0;
}
