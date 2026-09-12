#include <cassert>

class S {
  int data_;

public:
  S(int data) : data_(data) {}
  S(const S &) = delete;
  S &operator=(const S &) = delete;
  S(S &&) = default;

  friend bool operator==(const S &x, const S &y) {
    return x.data_ == y.data_;
  }
  friend bool operator<(const S &x, const S &y) {
    return x.data_ < y.data_;
  }
};

int main() {
  S a(1), b(2), c(1);
  assert(a == c);
  assert(a < b);
  assert(!(b < a));
  return 0;
}
