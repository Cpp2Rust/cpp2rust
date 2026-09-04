// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

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
  return 0;
}
