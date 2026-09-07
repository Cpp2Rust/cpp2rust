// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <vector>

int main() {
  std::vector<int> v1;
  v1.push_back(1);
  v1.push_back(2);
  v1.push_back(3);

  int sum = 0;

  for (auto x : v1)
    sum += ++x;
  for (const auto x : v1)
    sum += x;
  for (auto &x : v1)
    x += 10;
  for (const auto &x : v1)
    sum += x;

  std::vector<int *> v2;
  v2.push_back(&v1[0]);
  v2.push_back(&v1[1]);
  v2.push_back(&v1[2]);

  for (auto p : v2)
    *p += 5;

  for (const auto p : v2)
    sum += *p;

  for (auto *p : v2)
    *p += 5;

  for (const auto *p : v2)
    sum += *p;

  assert(sum == 168);
  return 0;
}
