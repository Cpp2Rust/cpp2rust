#include <cassert>
#include <utility>

enum Overload {
  kLvalueOverload = 1,
  kRvalueOverload = 2,
  kIntLvalueOverload = 3,
  kIntRvalueOverload = 4,
};

struct Tracked {
  int v;
  int copies;
  int moves;
  Tracked(int v) : v(v), copies(0), moves(0) {}
  Tracked(const Tracked &o) : v(o.v), copies(o.copies + 1), moves(o.moves) {}
  Tracked(Tracked &&o) : v(o.v), copies(o.copies), moves(o.moves + 1) {
    o.v = 0;
  }
};

static Overload chosen_overload(const Tracked &) { return kLvalueOverload; }

static Overload chosen_overload(Tracked &&) { return kRvalueOverload; }

static Overload chosen_overload(const int &) { return kIntLvalueOverload; }

static Overload chosen_overload(int &&) { return kIntRvalueOverload; }

static Tracked copy_or_move_into_param(Tracked t) { return t; }

int main() {
  Tracked a(5);
  assert(chosen_overload(std::forward<Tracked &>(a)) == kLvalueOverload);
  assert(a.v == 5);

  assert(chosen_overload(std::forward<Tracked &&>(a)) == kRvalueOverload);
  assert(a.v == 5);

  Tracked b(6);
  Tracked moved = copy_or_move_into_param(std::forward<Tracked &&>(b));
  assert(moved.v == 6);
  assert(moved.copies == 0);
  assert(moved.moves == 2);
  assert(b.v == 0);

  Tracked c(7);
  Tracked copied = copy_or_move_into_param(std::forward<Tracked &>(c));
  assert(copied.v == 7);
  assert(copied.copies == 1);
  assert(copied.moves == 1);
  assert(c.v == 7);

  int i = 8;
  assert(chosen_overload(std::forward<int &>(i)) == kIntLvalueOverload);
  assert(chosen_overload(std::forward<int>(i)) == kIntRvalueOverload);
  assert(chosen_overload(std::forward<int &&>(i)) == kIntRvalueOverload);
  assert(i == 8);

  return 0;
}
