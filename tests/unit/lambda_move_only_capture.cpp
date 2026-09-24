// no-compile
#include <assert.h>
#include <utility>

struct Owner {
  int *p;
  Owner(int v) : p(new int(v)) {}
  Owner(const Owner &) = delete;
  Owner(Owner &&o) : p(o.p) { o.p = nullptr; }
  ~Owner() { delete p; }
  auto take() {
    return [self = std::move(*this)]() { return *self.p; };
  }
};

int main() {
  Owner o(5);
  auto f = [h = std::move(o)]() { return *h.p; };
  assert(o.p == nullptr);
  assert(f() == 5);

  auto g = std::move(f);
  assert(g() == 5);

  int total = 0;
  auto consume = [h = Owner(7), &total]() mutable {
    total += *h.p;
    *h.p = 0;
  };
  consume();
  consume();
  assert(total == 7);

  Owner o2(9);
  auto t = o2.take();
  assert(o2.p == nullptr);
  assert(t() == 9);

  return 0;
}
