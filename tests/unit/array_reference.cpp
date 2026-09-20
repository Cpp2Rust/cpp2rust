#include <cassert>

int len(const char (&s)[5]) {
  int n = 0;
  while (s[n] != '\0') {
    ++n;
  }
  return n;
}

int len5(const char *s) {
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

int sum_ptr(const int *p) { return p[0] + p[1] + p[2]; }

int sum_decayed(const int (&a)[3]) { return sum_ptr(a); }

void bump_ptr(int *p) { p[0] += 1; }

void bump_decayed(int (&a)[3]) { bump_ptr(a); }

void fill_and_sum(int (&a)[3], int v, int &out) {
  fill(a, v);
  out = sum_twice(a);
}

const char (&pick(const char (&s)[5]))[5] { return s; }

struct Point {
  int x;
  int y;
};

int sum_points(const Point (&p)[2]) {
  return p[0].x + p[0].y + p[1].x + p[1].y;
}

void shift_points(Point (&p)[2], int d) {
  p[0].x += d;
  p[1].y += d;
}

int total_len(const char *(&names)[2]) {
  return len5(names[0]) + len5(names[1]);
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
  const char (&lit)[5] = "beta";
  assert(len(lit) == 4);
  assert(pick("beta")[0] == 'b');
  assert(len(pick(buf)) == 4);
  Point pts[2] = {{1, 2}, {3, 4}};
  assert(sum_points(pts) == 10);
  shift_points(pts, 10);
  assert(pts[0].x == 11);
  assert(pts[1].y == 14);
  assert(sum_points(pts) == 30);
  assert(sum_decayed(arr) == sum(arr));
  bump_decayed(arr);
  assert(arr[0] == 3);

  const char *names[2] = {"ab", "cde"};
  assert(total_len(names) == 5);
  return 0;
}
