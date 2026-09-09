// ADDITIONAL_COMPILE_FLAGS: -std=c++20
#include <cassert>
#include <concepts>

template <typename T>
concept Small = sizeof(T) <= 4;

static_assert(Small<int>);

template <typename T> bool is_small() { return Small<T>; }

template <typename T> int pick(T x) {
  if (std::integral<T> && Small<T>) {
    return 1;
  }
  return 2;
}

int main() {
  static_assert(!Small<long>);
  assert(is_small<char>());
  assert(!is_small<double>());
  assert(pick(1) == 1);
  assert(pick(1L) == 2);
  assert(pick(1.0f) == 2);
  return 0;
}
