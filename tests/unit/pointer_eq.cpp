#include <cassert>
#include <cstdint>

int main() {
  // Same variable, same pointer.
  int x = 5;
  const int *p1 = &x;
  const int *p2 = &x;
  assert(p1 == p2);

  // Different variables, different pointers.
  int y = 5;
  const int *p3 = &y;
  assert(p1 != p3);

  // Array pointer equals pointer to first element.
  int arr[] = {1, 2, 3};
  int *p = arr;
  assert(p == arr);
  assert(p + 1 == &arr[1]);
  assert(p + 2 == &arr[2]);

  // reinterpret_cast roundtrip preserves equality.
  int val = 42;
  int *orig = &val;
  unsigned char *as_bytes = reinterpret_cast<unsigned char *>(orig);
  int *back = reinterpret_cast<int *>(as_bytes);
  assert(orig == back);

  // reinterpret_cast roundtrip on array.
  unsigned char *arr_bytes = reinterpret_cast<unsigned char *>(arr);
  int *arr_back = reinterpret_cast<int *>(arr_bytes);
  assert(arr_back == arr);
  assert(arr_back + 1 == &arr[1]);

  return 0;
}
