// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <memory>

void change(std::unique_ptr<int> &p) {
  std::unique_ptr<int> q = std::make_unique<int>(7);
  p = std::move(q);
}

int main() {
  std::unique_ptr<int> a = std::make_unique<int>(5);
  change(a);
  assert(*a == 7);
  return 0;
}
