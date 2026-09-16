#include <assert.h>

template <typename T> int get(T t) { return t.x; }

namespace ns {
template <typename T> int twice(T t) { return t.x * 2; }
}

int main() {
  struct Local {
    int x;
  };
  Local l{7};
  assert(get(l) == 7);
  assert(ns::twice(l) == 14);
  return 0;
}
