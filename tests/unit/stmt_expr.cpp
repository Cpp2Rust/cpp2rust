#include <cassert>

int main() {
  int x = __extension__({
    int a = 1;
    int b = 2;
    a + b;
  });
  assert(x == 3);

  int counter = 0;
  int y = ({
    counter++;
    counter * 10;
  });
  assert(y == 10);
  assert(counter == 1);

  int z = ({
    int v = 5;
    if (v > 0) {
      v = v * 2;
    }
    v;
  });
  assert(z == 10);

  assert(({
      int inner = ({
          int a = 100;
          a;
      });
      inner;
  }) == 100);
  return 0;
}
