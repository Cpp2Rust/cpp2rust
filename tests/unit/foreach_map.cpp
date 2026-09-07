// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <map>

int main() {
  std::map<int, double> m;
  for (int i = 0, k = 100; i < 100; ++i, --k) {
    m[i] = k / 2.0;
  }
  double sum = 0;
  for (const auto &i : m) {
    sum += i.second;
  }
  for (auto i : m) {
    sum += i.first;
  }
  assert(sum == 7475);
  return 0;
}
