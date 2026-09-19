#include <cassert>
#include <new>

int main() {
  auto a = static_cast<int*>(::operator new(sizeof(int)));
  *a = 42;
  assert(*a == 42);
  ::operator delete(a);

  auto arr = static_cast<int*>(::operator new[](sizeof(int) * 2));
  arr[0] = 0;
  arr[1] = 1;
  assert(arr[0] + arr[1] == 1);
  ::operator delete[](arr);

  return 0;
}
