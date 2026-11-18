// panic: refcount
// nondet-result: unsafe

// Valid C++: the temporary lives until H's constructor returns. Both
// translations convert the argument in a block of its own, and that drops the
// temporary.

#include <cstdio>
#include <vector>

struct Outer {
  std::vector<int> values;
};

static Outer make() { return {{1, 2, 3, 4}}; }

struct H {
  int last;
  H(const std::vector<int> &values) : last(values[3]) {}
};

int main() {
  H h(make().values);
  printf("%d\n", h.last);
  return 0;
}
