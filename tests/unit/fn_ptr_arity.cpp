#include <assert.h>

int foo(int a1, int a2, int a3, int a4, int a5, int a6, int a7, int a8, int a9,
        int a10, int a11, int a12, int a13, int a14) {
  return 22;
}

int wide(int a1, int a2, int a3, int a4, int a5, int a6, int a7, int a8,
         int a9, int a10, int a11, int a12, int a13, int a14, int a15,
         int a16, int a17, int a18, int a19, int a20, int a21, int a22) {
  return a1 + a22;
}

int main() {
  auto f = &foo;
  assert(f(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14) == 22);

  auto w = &wide;
  assert(w(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
           20, 21, 22) == 23);

  auto l = [](int a1, int a2, int a3, int a4, int a5, int a6, int a7, int a8,
              int a9, int a10, int a11, int a12, int a13, int a14, int a15,
              int a16, int a17, int a18) { return a1 * a18; };
  assert(l(2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9) == 18);
  return 0;
}
