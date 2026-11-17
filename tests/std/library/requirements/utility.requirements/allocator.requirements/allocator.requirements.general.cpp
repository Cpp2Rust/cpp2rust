// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <cstddef>
#include <functional>
#include <map>
#include <memory>
#include <utility>

// N4950 [allocator.requirements.general]/18: if Allocator is SomeAllocator<T>
// and does not supply a rebind member template, rebind_alloc<U> is
// SomeAllocator<U>.
namespace p18_rebind_to_container_node {
template <class T> struct Alloc {
  using value_type = T;

  Alloc() = default;

  template <class U> Alloc(const Alloc<U> &) {}

  T *allocate(std::size_t n) { return std::allocator<T>().allocate(n); }

  void deallocate(T *p, std::size_t n) {
    std::allocator<T>().deallocate(p, n);
  }
};

void test() {
  std::map<int, int, std::less<int>, Alloc<std::pair<const int, int>>> m;
  m[1] = 2;
  assert(m[1] == 2);
  assert(m.size() == 1);
}
} // namespace p18_rebind_to_container_node

int main() {
  p18_rebind_to_container_node::test();
  return 0;
}
