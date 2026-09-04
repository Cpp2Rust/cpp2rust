#include <cassert>

// This tests that cpp2rust respects the lazy instantiation of templates. On
// Box<int> and Box<Point> declarations, only the class template and data
// memebers are instantiated.
//
// Then, each used method is lazyly instantiated when it's used, i.e. twice()
// is not instantiated for Box<Point>.

struct Point {
  int x;
};

template <typename T> struct Box {
  T val;
  T get() { return val; }

  // This only works for T's that have operator+.
  // int is fine, Point is not.
  T twice() { return val + val; }
};

int main() {
  Box<int> i = {3};
  assert(i.twice() == 6);

  Box<Point> p = {{4}};
  assert(p.get().x == 4);

  return 0;
}
