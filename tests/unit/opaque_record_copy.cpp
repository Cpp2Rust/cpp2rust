#include <cassert>

class Probe {
public:
  Probe &operator++();
};

template <class T> struct Wrapper {
  T base_;
  int tag;
};

int main() {
  Wrapper<Probe> a{};
  a.tag = 3;
  Wrapper<Probe> b = a;
  assert(b.tag == 3);
  return 0;
}
