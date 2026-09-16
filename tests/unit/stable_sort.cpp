#include <algorithm>

int main() {
  int arr1[] = {5, 2, 8, 1, 3};
  std::stable_sort(arr1, arr1 + 5, [](int x, int y) { return x < y; });
  return 0;
}
