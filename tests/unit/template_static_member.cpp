#include <cassert>

template <class T> struct Static {
  static T s;
};

template <class T> T Static<T>::s = 55;

int main() {
  Static<int>::s = 22;
  Static<char>::s = 33;

  assert(Static<int>::s == 22);
  assert(Static<char>::s == 33);
  assert(Static<long>::s == 55);

  return 0;
}
