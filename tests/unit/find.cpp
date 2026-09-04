// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <algorithm>
#include <cassert>
#include <map>
#include <vector>

int main() {
  std::vector<int> v;
  v.push_back(1);
  v.push_back(2);
  v.push_back(3);
  auto v_begin = v.begin();
  auto v_end = v.end();
  auto it = std::find(v_begin, v_end, 2);
  bool v_result_true = it != v.end();
  std::map<int, double> m;
  m[1] = 1;
  m[2] = 2;
  m[3] = 3;
  auto m_begin = m.begin();
  auto m_end = m.end();
  bool m_result_true = m_begin != m_end;
  assert(v_result_true && m_result_true &&
         std::find(v.begin(), v.begin(), 2) == v.begin());
  return 0;
}
