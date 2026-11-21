// ADDITIONAL_COMPILE_FLAGS: -std=c++14

#include <cstdio>
#include <vector>

using Rows = std::vector<std::vector<int>>;

Rows *select_rows(Rows *rows, int *calls) {
  ++*calls;
  (*rows)[0][2] = 9;
  return rows;
}

int main() {
  for (int bound = 0; bound < 2; ++bound) {
    Rows rows{{1, 2, 3}};
    rows.reserve(2);
    int calls = 0;
    if (bound) {
      Rows *receiver = select_rows(&rows, &calls);
      receiver->emplace_back(rows[0].data(), rows[0].data() + 3);
    } else {
      select_rows(&rows, &calls)
          ->emplace_back(rows[0].data(), rows[0].data() + 3);
    }
    std::printf("source=%d inserted=%d calls=%d\n", rows[0][2], rows[1][2],
                calls);
  }
}
