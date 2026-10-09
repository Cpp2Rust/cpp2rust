#include <cassert>
#include <cstdarg>

static int sum(int n, ...) {
  struct Guard {
    va_list &ap;
    bool active;
    Guard(va_list &val) : ap(val), active(true) {}
    ~Guard() {
      if (active) {
        va_end(ap);
      }
    }
  };
  va_list ap;
  va_start(ap, n);
  Guard guard(ap);
  int total = 0;
  for (int i = 0; i < n; ++i) {
    total += va_arg(guard.ap, int);
  }
  return total;
}

static int sum_ptr(int n, ...) {
  struct Cursor {
    va_list *ap;
  };
  va_list ap;
  va_start(ap, n);
  Cursor cursor{&ap};
  int total = 0;
  for (int i = 0; i < n; ++i) {
    total += va_arg(*cursor.ap, int);
  }
  va_end(ap);
  return total;
}

int main() {
  assert(sum(3, 1, 2, 3) == 6);
  assert(sum_ptr(2, 4, 5) == 9);
  return 0;
}
