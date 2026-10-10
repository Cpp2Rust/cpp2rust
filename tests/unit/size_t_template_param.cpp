#include <cassert>
#include <cstddef>

template <class T, size_t N> static T array_ref(T (&a)[N]) {
  a[0] += 1;
  return a[N - 1];
}

template <class T> struct PtrCtor {
  T v;
  PtrCtor(const T *p) : v(p[1]) {}
};

template <class T> struct RefCtor {
  T v;
  RefCtor(const T &x) : v(x + 1) {}
};

template <long long N> static long long neg_ll() { return N; }

template <char C> static char neg_char() { return C; }

template <int N> static int neg_int() { return N; }

int main() {
  size_t a1[] = {1, 2, 3};
  assert(array_ref(a1) == 3);
  assert(a1[0] == 2);

  size_t a2[] = {4, 5};
  PtrCtor<size_t> pc(a2);
  assert(pc.v == 5);

  size_t v1 = 6;
  RefCtor<size_t> rc(v1);
  assert(rc.v == 7);

  assert(neg_ll<-9000000>() == -9000000);
  assert(neg_char<'\x80'>() == '\x80');
  assert(neg_int<-1>() == -1);
  return 0;
}
