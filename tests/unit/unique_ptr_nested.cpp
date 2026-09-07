// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// Tests chained arrow through nested unique_ptrs and cross-deref assignment.
#include <cassert>
#include <memory>

struct Inner {
  int x;
  int y;
};

struct Outer {
  std::unique_ptr<Inner> inner;
};

int main() {
  auto o =
      std::make_unique<Outer>(Outer{std::make_unique<Inner>(Inner{10, 20})});
  o->inner->x += 5;
  int sum = o->inner->x + o->inner->y;

  auto a = std::make_unique<int>(100);
  auto b = std::make_unique<int>(0);
  *b = *a;

  assert(sum + *b == 135);
  return 0;
}
