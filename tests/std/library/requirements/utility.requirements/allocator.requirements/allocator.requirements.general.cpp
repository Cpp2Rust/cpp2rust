// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <cstddef>
#include <functional>
#include <map>
#include <memory>
#include <utility>

// N4950 [allocator.requirements.general]/16: X::rebind<U>::other is the
// allocator of U that corresponds to X.
namespace p16_rebind_member {
template <class T> struct Alloc {
  using value_type = T;

  int id = 0;

  Alloc() = default;

  template <class U> Alloc(const Alloc<U> &other) : id(other.id) {}

  template <class U> struct rebind {
    using other = Alloc<U>;
  };

  T *allocate(std::size_t n) { return std::allocator<T>().allocate(n); }

  void deallocate(T *p, std::size_t n) { std::allocator<T>().deallocate(p, n); }
};

template <class T, class U>
bool operator==(const Alloc<T> &a, const Alloc<U> &b) {
  return a.id == b.id;
}

void test() {
  std::map<int, int, std::less<int>, Alloc<std::pair<const int, int>>> m;
  std::map<int, int, std::less<int>, Alloc<std::pair<const int, int>>> n;
  m[1] = 2;
  n = std::move(m);
  assert(n[1] == 2);
  assert(n.size() == 1);
}
} // namespace p16_rebind_member

// N4950 [allocator.requirements.general]/18: if Allocator is SomeAllocator<T>
// and does not supply a rebind member template, rebind_alloc<U> is
// SomeAllocator<U>.
namespace p18_rebind_to_container_node {
template <class T> struct Alloc {
  using value_type = T;

  Alloc() = default;

  template <class U> Alloc(const Alloc<U> &) {}

  T *allocate(std::size_t n) { return std::allocator<T>().allocate(n); }

  void deallocate(T *p, std::size_t n) { std::allocator<T>().deallocate(p, n); }
};

void test() {
  std::map<int, int, std::less<int>, Alloc<std::pair<const int, int>>> m;
  m[1] = 2;
  assert(m[1] == 2);
  assert(m.size() == 1);
}
} // namespace p18_rebind_to_container_node

int main() {
  p16_rebind_member::test();
  p18_rebind_to_container_node::test();
  return 0;
}
