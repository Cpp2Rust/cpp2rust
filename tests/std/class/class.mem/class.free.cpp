// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <cstddef>
#include <new>

// N4950 [class.free]/1: any allocation function for a class T is a static
// member (even if not explicitly declared static).
namespace p1_allocation_function_is_static {
int allocations = 0;

struct Allocates {
  void *operator new(std::size_t size) {
    ++allocations;
    return ::operator new(size);
  }

  void *operator new[](std::size_t size) {
    allocations += 10;
    return ::operator new[](size);
  }
};

void test() {
  auto one = static_cast<int *>(Allocates::operator new(sizeof(int)));
  *one = 7;
  assert(*one == 7);
  assert(allocations == 1);
  ::operator delete(one);

  auto many = static_cast<int *>(Allocates::operator new[](sizeof(int) * 2));
  many[0] = 1;
  many[1] = 2;
  assert(many[0] + many[1] == 3);
  assert(allocations == 11);
  ::operator delete[](many);
}
} // namespace p1_allocation_function_is_static

// N4950 [class.free]/3: any deallocation function for a class X is a static
// member (even if not explicitly declared static).
namespace p3_deallocation_function_is_static {
int deallocations = 0;

struct Deallocates {
  void operator delete(void *ptr) {
    ++deallocations;
    ::operator delete(ptr);
  }

  void operator delete[](void *ptr) {
    deallocations += 10;
    ::operator delete[](ptr);
  }
};

void test() {
  auto one = static_cast<int *>(::operator new(sizeof(int)));
  Deallocates::operator delete(one);
  assert(deallocations == 1);

  auto many = static_cast<int *>(::operator new[](sizeof(int) * 2));
  Deallocates::operator delete[](many);
  assert(deallocations == 11);
}
} // namespace p3_deallocation_function_is_static

int main() {
  p1_allocation_function_is_static::test();
  p3_deallocation_function_is_static::test();
  return 0;
}
