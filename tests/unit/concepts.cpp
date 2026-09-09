// ADDITIONAL_COMPILE_FLAGS: -std=c++20
#include <cassert>
#include <concepts>

template <typename T>
concept Small = sizeof(T) <= 4;

static_assert(Small<int>);

template <typename T>
concept HasSize = requires(T t) {
  { t.size() } -> std::same_as<int>;
};

struct Sized {
  int size() { return 4; }
};

template <typename T> bool is_small() { return Small<T>; }

template <typename T> bool has_size() { return requires(T t) { t.size(); }; }

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
  assert(HasSize<Sized>);
  assert(!HasSize<int>);
  assert(has_size<Sized>());
  assert(!has_size<int>());
  assert(pick(1) == 1);
  assert(pick(1L) == 2);
  assert(pick(1.0f) == 2);
  return 0;
}
