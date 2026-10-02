// ADDITIONAL_COMPILE_FLAGS: -std=c++17
#include <assert.h>

struct Pair {
  int first;
  int second;
};

int main() {
  Pair arr[3] = {{1, 2}, {3, 4}, {5, 6}};

  for (auto &[x, y] : arr) {
    x += y;
  }
  assert(arr[0].first == 3);
  assert(arr[2].first == 11);

  for (auto [x, y] : arr) {
    x = 0;
    y = 0;
  }
  assert(arr[1].first == 7);
  assert(arr[1].second == 4);

  int sum = 0;
  for (const auto &[x, y] : arr) {
    sum += x * y;
  }
  assert(sum == 3 * 2 + 7 * 4 + 11 * 6);
  return 0;
}
