#include <atomic>
#include <cassert>

int main() {
  std::atomic<int> a{};
  int v = a;
  assert(v == 0);
  return 0;
}
