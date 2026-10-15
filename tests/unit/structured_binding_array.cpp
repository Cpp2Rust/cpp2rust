// ADDITIONAL_COMPILE_FLAGS: -std=c++17
#include <assert.h>

int main() {
  int arr[3] = {1, 2, 3};

  auto [a, b, c] = arr;
  a = 10;
  assert(a == 10);
  assert(b == 2);
  assert(c == 3);
  assert(arr[0] == 1);

  auto &[x, y, z] = arr;
  x = 7;
  z += y;
  assert(arr[0] == 7);
  assert(arr[2] == 5);
  return 0;
}
