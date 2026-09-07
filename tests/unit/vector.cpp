// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <assert.h>
#include <vector>

void copy(std::vector<int> copy_vector) {}

int main() {
  std::vector<int> v1;
  assert(v1.size() == 0);
  assert(v1.empty());
  v1.push_back(1);
  assert(!v1.empty());
  v1.pop_back();
  assert(v1.empty());
  auto s1 = v1.size();
  v1.resize(100);
  assert(v1.size() == 100);
  assert(v1[99] == 0);
  v1[0] = 40;
  v1[99] = 50;
  assert(v1[0] == 40);
  assert(v1[99] == 50);
  std::vector<int> v2;
  assert(v2.size() == 0);
  v2.push_back(1);
  v2.push_back(2);
  v2.push_back(3);
  assert(v2.size() == 3);
  v2.erase(v2.begin());
  assert(v2.size() == 2);
  assert(v2[0] == 2);
  assert(v2[1] == 3);
  v2.insert(v2.begin(), 100);
  copy(v2);
  assert(v2.size() == 3);
  assert(v2[0] == 100);
  assert(v2[1] == 2);
  assert(v2[2] == 3);
  auto s2 = v2.size();
  std::vector<int> v3(100, 1);
  assert(v3.size() == 100);
  for (int i = 0; i < 100; ++i)
    assert(v3[i] == 1);
  std::vector<int *> v4(100);
  assert(v4.size() == 100);
  for (unsigned i = 0; i < v4.size(); ++i)
    assert(v4[i] == nullptr);
  std::vector<const int *> v5(100);
  assert(v5.size() == 100);
  for (unsigned i = 0; i < v5.size(); ++i)
    assert(v5[i] == nullptr);
  std::vector<double> v6(s2, 2.0);
  assert(v6.size() == s2);
  for (unsigned i = 0; i < s2; ++i)
    assert(v6[i] == 2.0);
  std::vector<std::pair<const int *, int>> v7(200);
  assert(v7.size() == 200);
  for (unsigned i = 0; i < 200; ++i)
    assert(v7[i].first == nullptr && v7[i].second == 0);
  const double *p1 = v6.data();
  assert(*p1 == 2.0);
  int *p2 = v3.data();
  assert(*p2 == 1);
  assert(v3[0] == 1);
  assert(v3[1] == 1);
  *p2 = 99.0;
  assert(*p2 == 99);
  assert(v3[0] == 99);
  assert(v3[1] == 1);
  ++p2;
  *p2 = 98;
  assert(v3[0] == 99);
  assert(v3[1] == 98);
  assert(v3.capacity() == 100);
  assert(v3.size() == 100);
  v3.reserve(200);
  assert(v3.capacity() == 200);
  assert(v3.size() == 100);
  v3.reserve(50);
  assert(v3.capacity() == 200);
  assert(v3.size() == 100);
  v3.reserve(200);
  assert(v3.capacity() == 200);
  assert(v3.size() == 100);
  v3.reserve(201);
  assert(v3.capacity() == 201);
  assert(v3.size() == 100);
  assert(v2.back() == 3);
  assert(v3.back() == 1);
  assert(v4.back() == nullptr);
  assert(v5.back() == nullptr);
  assert(v6.back() == 2.0);
  double &ref0 = v6.back();
  ref0 = 5.0;
  assert(v6.back() == 5.0);
  double x0 = v6.back();
  assert(x0 == 5.0);
  x0 = 6.0;
  assert(v6.back() == 5.0);
  int idx = 0;
  assert(v6.at(idx) == 2.0);
  assert(v6.at(s2 - 1) == 5.0);
  double &ref1 = v6.at(s2 - 1);
  ref1 += 1.5;
  assert(v6.at(s2 - 1) == 6.5);
  double x1 = v6.at(s2 - 1);
  assert(x1 == 6.5);
  x1 -= 1.5;
  assert(v6.at(s2 - 1) == 6.5);
  assert(s1 + s2 + v2.at(0) == 103);
  return 0;
}
