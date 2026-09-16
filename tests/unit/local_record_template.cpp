#include <assert.h>

template <typename T> int get(T t) { return t.x; }

namespace ns {
template <typename T> int twice(T t) { return t.x * 2; }
}

template <typename T> int wrap(T v) {
  struct Local {
    T x;
  };
  Local l{v};
  return get(l);
}

int other() {
  struct Local {
    long x;
    long y;
  };
  Local l{3, 4};
  return get(l) + (int)l.y;
}

int main() {
  struct Local {
    int x;
  };
  Local l{7};
  assert(get(l) == 7);
  assert(ns::twice(l) == 14);
  assert(other() == 7);
  assert(wrap(5) == 5);
  assert(wrap(6L) == 6);
  return 0;
}
