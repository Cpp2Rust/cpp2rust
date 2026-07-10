// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <functional>
#include <iostream>

struct Point {
  int x;
  int y;
};

void set(std::reference_wrapper<int> ref, int val) { ref.get() = val; }

int read(std::reference_wrapper<int> ref) {
  int &r = ref.operator int &();
  return r;
}

int main() {
  int i1 = 10;
  std::reference_wrapper<int> ref_1(i1);
  ref_1.get() = 20;
  int &i2 = ref_1.operator int &();
  i2 += 5;
  std::cout << i1 << '\n';

  int i3 = 1;
  int i4 = 2;
  std::reference_wrapper<int> ref_3(i3);
  std::reference_wrapper<int> ref_4(i4);
  ref_3.get() = ref_4.get();
  std::cout << i3 << ' ' << i4 << '\n';

  set(ref_1, 99);
  std::cout << i1 << ' ' << read(ref_1) << '\n';

  Point point{3, 4};
  std::reference_wrapper<Point> point_ref(point);
  point_ref.get().x = 30;
  point_ref.operator Point &().y = 40;
  std::cout << point.x << ' ' << point.y << '\n';

  return 0;
}
