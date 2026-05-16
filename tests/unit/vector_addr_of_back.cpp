#include <assert.h>
#include <vector>

int main() {
  std::vector<std::vector<int>> outer;
  std::vector<int> inner;
  outer.push_back(inner);
  auto *sink = &outer.back();
  assert(sink->size() == 0);
  return 0;
}
