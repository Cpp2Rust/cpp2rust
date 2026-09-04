// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <memory>

std::unique_ptr<int> fn(std::unique_ptr<int> u) {
  *u = 10;
  return u;
}

int main() {
  auto f = std::make_unique<int>(8);
  *f = 9;

  int *f_ptr1 = f.get();
  *f_ptr1 = 10;

  int *f_ptr2 = &*f;
  *f_ptr2 = 11;

  f = std::make_unique<int>(9);
  f = fn(std::move(f));

  assert(*f == 10);
  return 0;
}
