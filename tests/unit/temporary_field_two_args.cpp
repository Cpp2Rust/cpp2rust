// panic: refcount

// Valid C++: both temporaries live until two() returns. The refcount
// translation binds each argument to a variable before the call, and that
// drops its temporary.

#include <cstdio>
#include <vector>

struct Inner {
  std::vector<int> values;
};

struct Outer {
  std::vector<int> values;
  Inner inner;
};

static Outer make() { return {{1, 2, 3}, {{4, 5, 6}}}; }

static int two(const std::vector<int> &a, const std::vector<int> &b) {
  return a[0] + b[2];
}

int main() {
  printf("%d\n", two(make().values, make().inner.values));
  return 0;
}
