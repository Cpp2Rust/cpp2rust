// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// panic-ub: refcount

#include <vector>

const int &foo(const int &a) { return a; }

int main() {
  std::vector<int> v = {1, 2};
  const int &b = foo(*v.begin());
  v.clear();
  return b;
}
