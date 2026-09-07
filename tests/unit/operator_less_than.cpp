// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

struct Pair {
  int x, y;
  bool operator<(const Pair &other) {
    return x < other.x || (x == other.x && y < other.y);
  }
};

int main() {
  Pair pair1 = {1, 2};
  Pair pair2 = {1, 3};
  assert(pair1 < pair2);
  return 0;
}
