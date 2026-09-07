// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <memory>

void change(std::unique_ptr<int> &n) {
  std::unique_ptr<int> m = std::make_unique<int>(20);
  n = std::move(m);
}

int main() {
  std::unique_ptr<int> n = std::make_unique<int>(10);
  change(n);
  assert(*n == 20);
  return 0;
}
