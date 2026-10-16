// ADDITIONAL_COMPILE_FLAGS: -std=c++20
#include <cassert>
#include <memory>

int main() {
  auto g = std::make_unique<int[]>(2);
  g[0] = 11;
  g[1] = 12;
  int *g_ptr = g.get();
  g_ptr[0] = 13;
  g_ptr[1] = 14;
  assert(g[0] + g[1] == 27);
  auto buf = std::make_unique<char[]>(16);
  std::declare_no_pointers(buf.get(), 16);
  buf[0] = 'x';
  std::undeclare_no_pointers(buf.get(), 16);
  assert(buf[0] == 'x');
  return 0;
}
