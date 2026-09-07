// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <vector>

int main() {
  [[maybe_unused]] auto x1 = 1;
  [[maybe_unused]] auto x2 = (short)2;
  [[maybe_unused]] auto x3 = (unsigned)4;
  std::vector<int> v;
  v.push_back(1);
  v.push_back(2);
  auto sum = 0;
  for (auto elem : v)
    sum += elem;
  assert(sum == 3);
  return 0;
}
