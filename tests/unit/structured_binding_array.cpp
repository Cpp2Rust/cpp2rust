// ADDITIONAL_COMPILE_FLAGS: -std=c++17
#include <array>
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

  std::array<int, 3> std_arr = {4, 5, 6};
  std::tuple_element<2, std::array<int, 3>>::type last = std::get<2>(std_arr);
  std::get<0>(std_arr) = 8;
  auto &[p, q, r] = std_arr;
  q += 10;
  assert(last == 6);
  assert(std_arr[0] == 8);
  assert(std_arr[1] == 15);
  assert(r == 6);
  return 0;
}
