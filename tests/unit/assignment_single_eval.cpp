// ADDITIONAL_COMPILE_FLAGS: -std=c++17
#include <cstdio>
#include <vector>

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

int global = 10;
int observe_global(int assigned) { return assigned + global; }
int observe(int assigned, const int *stored) { return assigned + *stored; }

void test_borrows() {
  std::printf("%d\n", observe_global(global = 99));
  std::printf("%d\n", observe_global(global += 5));

  int local = 10;
  std::printf("%d\n", observe(local = 99, &local));
  std::printf("%d\n", observe(local += 5, &local));

  struct Record {
    int value;
  };
  Record record{10};
  std::printf("%d\n", observe(record.value = 99, &record.value));
  std::printf("%d\n", observe(record.value += 5, &record.value));

  std::vector<int> values(1, 10);
  std::printf("%d\n", observe(values[0] = 99, &values[0]));
  std::printf("%d\n", observe(values[0] += 5, &values[0]));
}

int main() {
  test<unsigned>();
  test<short>();
  test<int>();
  test_borrows();
  return 0;
}
