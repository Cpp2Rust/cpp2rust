// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <vector>

int main() {
  std::vector<int> v;
  v.push_back(1);
  v.push_back(2);
  v.push_back(3);
  int square = 0;
  for (auto e1 : v)
    for (auto e2 : v)
      square += e1 * e2;
  for (auto &e1 : v)
    for (const auto &e2 : v)
      square += e1 * e2;
  for (const auto &e1 : v)
    for (auto &e2 : v)
      square += e1 * e2;
  for (auto &e1 : v)
    for (auto &e2 : v)
      square += e1 * e2;
  std::vector<std::vector<int>> m;
  std::vector<int> v1;
  m.push_back(std::move(v1));
  std::vector<int> v2;
  m.push_back(std::move(v2));
  std::vector<int> v3;
  m.push_back(std::move(v3));
  for (auto &row : m)
    for (auto &col : row)
      square += col;
  assert(square == 144);
  return 0;
}
