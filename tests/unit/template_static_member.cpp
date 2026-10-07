#include <cassert>

template <class T> struct Static {
  static T s;
};

template <class T> T Static<T>::s = 55;

template <class T> struct Range {
  static T lo;
  static T hi;
};

#define DEFINE_RANGE(T, a, b)                                                   \
  template <> T Range<T>::lo = a;                                              \
  template <> T Range<T>::hi = b

DEFINE_RANGE(int, -1, 1);
DEFINE_RANGE(long, -2, 2);

int main() {
  Static<int>::s = 22;
  Static<char>::s = 33;

  assert(Static<int>::s == 22);
  assert(Static<char>::s == 33);
  assert(Static<long>::s == 55);
  assert(Range<int>::lo == -1 && Range<int>::hi == 1);
  assert(Range<long>::lo == -2 && Range<long>::hi == 2);

  return 0;
}
