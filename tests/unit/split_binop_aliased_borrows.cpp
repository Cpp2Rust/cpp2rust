// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <vector>

int main() {
  std::vector<int> v = {1, 2};
  auto p = v.begin();
  const auto &r = v[1];
  // In refcount mode, both p (pointer) and r (reference) are translated as Ptr.
  // Thus, the below assignment must be written as:
  //   let __rhs = r;
  //   *p = __rhs;
  // To avoid borrow errors.
  *p = r;
  assert(v[0] == 2);
  return 0;
}
