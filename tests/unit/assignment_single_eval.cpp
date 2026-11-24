// ADDITIONAL_COMPILE_FLAGS: -std=c++17
#include <cstdio>

template <class T> void test() {
  T a[3] = {10, 20, 30};
  int i = 0;
  a[i++] += 5;
  std::printf("void %d %d %d\n", i, int(a[0]), int(a[1]));

  i = 0;
  T sum = (a[i++] += 2);
  std::printf("compound %d %d %d %d\n", i, int(sum), int(a[0]), int(a[1]));

  i = 0;
  T assigned = (a[i++] = 99);
  std::printf("simple %d %d %d %d\n", i, int(assigned), int(a[0]), int(a[1]));
}

int main() {
  test<unsigned>();
  test<short>();
  test<int>();
  return 0;
}
