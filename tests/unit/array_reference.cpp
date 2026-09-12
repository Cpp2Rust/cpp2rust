#include <cassert>

int len(const char (&s)[5]) {
  int n = 0;
  while (s[n] != '\0') {
    ++n;
  }
  return n;
}

int sum(const int (&a)[3]) { return a[0] + a[1] + a[2]; }

void fill(int (&a)[3], int v) {
  for (int i = 0; i < 3; ++i) {
    a[i] = v;
  }
}

int sum_twice(const int (&a)[3]) { return sum(a) + sum(a); }

void fill_and_sum(int (&a)[3], int v, int &out) {
  fill(a, v);
  out = sum_twice(a);
}

int main() {
  assert(len("beta") == 4);
  char buf[5] = "abcd";
  assert(len(buf) == 4);
  int arr[3] = {1, 2, 3};
  assert(sum(arr) == 6);
  fill(arr, 7);
  assert(sum(arr) == 21);
  assert(sum_twice(arr) == 42);
  int out = 0;
  fill_and_sum(arr, 2, out);
  assert(out == 12);
  assert(arr[0] == 2);
  return 0;
}
