// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <vector>

int main() {
  std::vector<int> v;
  v.push_back(10);
  v.front() += 5;
  assert(v.front() == 15);
  return v.front();
}
