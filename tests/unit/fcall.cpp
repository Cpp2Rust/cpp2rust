// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

double f2(double x, double y) { return x - y; }

double f3(double x, double y, double z) { return f2(x, y) + z; }

double f1(double x, double y) {
  double z1 = f2(x, y);
  if (f2(z1, y) < 0) {
    double z2 = -f3(z1, f2(x, y), y);
    return f2(f2(z2, f3(z1, z2, x)), y);
  }
  return f2(z1, x);
}

int main() {
  assert(f1(1.0, 2.0) == -6);
  return 0;
}
