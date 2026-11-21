// ADDITIONAL_COMPILE_FLAGS: -std=c++14

#include <cstdio>
#include <vector>

using Rows = std::vector<std::vector<int>>;

Rows *select_rows(Rows *rows, int *calls) {
  ++*calls;
  return rows;
}

int main() {
  Rows rows{{1, 2, 3}};
  rows.reserve(2);
  int calls = 0;
  Rows *alias = &rows;
  select_rows(alias, &calls)
      ->emplace_back((*alias)[0].data(), (*alias)[0].data() + 3);
  std::printf("calls=%d size=%zu last=%d\n", calls, rows.size(), rows[1][2]);
}
