// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

struct X {
  int x;
};

int func() { return 42; }

int main() {
  int x;
  int *p;
  int &g = x;
  int *q = &x;
  int *z = p;
  X xx, *zz = &xx;
  xx.x = 1;
  q = &xx.x;
  q = &zz->x;
  zz->x = 2;
  X ww = xx;
  ww = xx;
  int aa = func();
  aa = func();
  return 0;
}
