// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <vector>

int main() {
  std::vector<int> v;
  for (int i = 0; i < 10; ++i)
    v.push_back(i);
  int sum = 0;
  for (int x : v)
    sum += x;
  assert(sum == 45);
  return 0;
}
