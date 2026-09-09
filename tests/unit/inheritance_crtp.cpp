#include <cassert>

template <class D> struct Counter {
  int n = 0;
  D &inc() {
    ++n;
    return static_cast<D &>(*this);
  }
};

struct Impl : Counter<Impl> {
  int twice() { return n * 2; }
};

int main() {
  Impl i;
  i.inc().inc();
  assert(i.twice() == 4);
  assert(i.inc().twice() == 6);
  return 0;
}
