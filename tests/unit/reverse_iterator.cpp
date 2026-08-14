// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <iterator>

struct Foo {
  int v;
  int get() const { return v; }
};

int main() {
  Foo a1[] = {{10}, {20}, {30}, {40}, {50}};

  std::reverse_iterator<Foo *> def;
  std::reverse_iterator<Foo *> first(a1 + 5);
  assert(first.base() == a1 + 5);

  std::reverse_iterator<Foo *>::reference ref = *first;
  assert(ref.get() == 50);
  assert(first->get() == 50);

  ++first;
  assert(first->get() == 40);

  std::reverse_iterator<Foo *> inc = first++;
  assert(inc->get() == 40);
  assert(first->get() == 30);

  std::reverse_iterator<Foo *>::difference_type n = 2;

  std::reverse_iterator<Foo *> plus = first + n;
  assert(plus->get() == 10);
  std::reverse_iterator<Foo *> minus = plus - n;
  assert(minus->get() == 30);
  assert(minus == first);
  return 0;
}
