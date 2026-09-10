#include <cassert>
#include <utility>

struct Chain {
  int v;
  Chain(int v) : v(v) {}
  Chain(const Chain &o) : v(o.v + 100) {}
  Chain(Chain &&o) : v(o.v + 1) { o.v = 0; }

  Chain &add(int n) & {
    v += n;
    return *this;
  }
  Chain &&add(int n) && {
    v += n;
    return std::move(*this);
  }
  Chain take() && { return std::move(*this); }
  Chain copy() const & { return *this; }
  Chain &&self() && { return std::move(*this); }
};

static int consume(Chain c) { return c.v; }

int main() {
  Chain a(1);
  a.add(1).add(1);
  assert(a.v == 3);

  Chain b0(5);
  Chain b = std::move(b0).add(1).add(1);
  assert(b.v == 8 && b0.v == 0);

  Chain c = Chain(10).take();
  assert(c.v == 11);

  Chain d = c.copy();
  assert(d.v == 111 && c.v == 11);

  Chain g(20);
  assert(consume(std::move(g).self()) == 21);

  Chain e(30);
  Chain f = std::move(e).take();
  assert(f.v == 31 && e.v == 0);
  return 0;
}
