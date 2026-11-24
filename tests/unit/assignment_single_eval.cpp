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

template <class T> void test_comma() {
  T x = 10;
  int i = 0;
  ((++i, x) += 5);
  std::printf("direct void %d %d\n", i, int(x));
  T result = ((++i, x) += 2);
  std::printf("direct compound %d %d %d\n", i, int(result), int(x));
  result = ((++i, x) = 99);
  std::printf("direct simple %d %d %d\n", i, int(result), int(x));

  T *p = &x;
  ((++i, *p) += 5);
  std::printf("indirect void %d %d\n", i, int(*p));
  result = ((++i, *p) += 2);
  std::printf("indirect compound %d %d %d\n", i, int(result), int(*p));
  result = ((++i, *p) = 99);
  std::printf("indirect simple %d %d %d\n", i, int(result), int(*p));
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
  test_comma<unsigned>();
  test_comma<short>();
  unsigned values[2] = {10, 20};
  unsigned (&ref)[2] = values;
  int i = 0;
  unsigned result = ((++i, ref)[0] += 5);
  std::printf("array reference %d %u %u\n", i, result, values[0]);
  test_borrows();
  return 0;
}
