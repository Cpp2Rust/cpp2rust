// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <iostream>
#include <memory>

int identity(int x) { return x; }

void swap_by_ptr(int *a, int *b) {
  int tmp = *a;
  *a = *b;
  *b = tmp;
}

void swap_by_ref(int &a, int &b) {
  int tmp = a;
  a = b;
  b = tmp;
}

int main() {
  // Local variable
  int x = 0;
  std::cout << x << '\n';

  // Pass and return by value
  int a = 1;
  int b = identity(a);
  std::cout << b << '\n';

  // Local aliasing
  int c = 2;
  int *p = &c;
  std::cout << *p << '\n';

  // Pass by pointer
  int d = 3;
  int e = 4;
  swap_by_ptr(&d, &e);

  // Pass by reference
  int f = 4;
  int g = 5;
  swap_by_ref(f, g);

  // New and delete
  int *h = new int(6);
  std::cout << *h << '\n';
  delete h;

  // Array new and delete
  int *i = new int[3]{7, 8};
  std::cout << i[0] << ' ' << i[1] << '\n';
  delete[] i;

  // Weird stuff
  swap_by_ptr(new int(7), new int(8));
  swap_by_ptr(new int(7) + 0, new int(8) + 0);

  swap_by_ref(*new int(9), *new int(10));
  swap_by_ref((new int(9))[0], (new int(10))[0]);

  // Unique pointer from raw pointer
  std::unique_ptr<int> j(new int(11));
  int *k = j.get();
  std::cout << *k << '\n';

  // Unique pointer from std::make_unique
  std::unique_ptr<int> l = std::make_unique<int>(11);
  int *m = l.get();
  std::cout << *m << '\n';

  assert(c == 2);
  return 0;
}
