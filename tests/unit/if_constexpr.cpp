#include <cassert>
#include <type_traits>

template <typename T> int classify(T x) {
  if constexpr (std::is_pointer_v<T>) {
    return *x;
  } else if constexpr (sizeof(T) > 4) {
    return 2;
  }
  return 1;
}

int keep_both(int x) {
  if constexpr (sizeof(int) > 2) {
    return x + 1;
  } else {
    return x - 1;
  }
}

template <typename T> unsigned long long widen(T v) {
  if (sizeof(T) > 4) {
    T hi = v;
    hi <<= 32;
    return hi;
  }
  return v;
}

int main() {
  int v = 7;
  assert(classify(&v) == 7);
  assert(classify(1L) == 2);
  assert(classify(1) == 1);
  assert(keep_both(1) == 2);
  assert(widen(1u) == 1);
  assert(widen(1ull) == 4294967296ull);
  return 0;
}
