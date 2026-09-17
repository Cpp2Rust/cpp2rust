#include <cstdio>
#include <vector>

int main() {
  std::vector<std::vector<int>> v;
  v.resize(2);
  v[0].resize(2);
  v[1].resize(1);

  v[0][0] = 1;
  v[0][1] = 5;
  v[1][0] = 6;

  for (auto &v2 : v) {
    for (auto &i : v2) {
      ++i;
    }
  }

  for (auto &v2 : v) {
    for (auto i : v2) {
      printf("%d\n", i + 3);
    }
  }

  for (auto v2 : v) {
    for (auto i : v2) {
      printf("%d\n", i);
    }
  }

  return 0;
}
