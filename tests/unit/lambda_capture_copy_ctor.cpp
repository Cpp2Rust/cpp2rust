// no-compile: refcount
// panic: unsafe
#include <assert.h>
#include <utility>

struct Counted {
  int copies;
  int moves;
  Counted() : copies(0), moves(0) {}
  Counted(const Counted &o) : copies(o.copies + 1), moves(o.moves) {}
  Counted(Counted &&o) : copies(o.copies), moves(o.moves + 1) {}
};

int drops = 0;

struct Dropped {
  Dropped() {}
  Dropped(const Dropped &) {}
  Dropped(Dropped &&) {}
  ~Dropped() { drops++; }
};

int main() {
  Counted c;
  auto f = [c]() { return c.copies * 10 + c.moves; };
  assert(f() == 10);

  auto g = f;
  assert(g() == 20);
  assert(f() == 10);

  auto h = std::move(f);
  assert(h() == 11);

  int returned = [c]() { return c; }().copies;
  assert(returned == 2);

  Counted arr[2];
  auto a = [arr]() { return arr[0].copies + arr[1].copies; };
  assert(a() == 2);

  auto a2 = a;
  assert(a2() == 4);

  {
    auto m = [d = Dropped()]() {};
    auto m2 = std::move(m);
  }
  assert(drops == 2);

  {
    auto k = [d = Dropped()]() {};
    auto k2 = k;
  }
  assert(drops == 4);

  return 0;
}
