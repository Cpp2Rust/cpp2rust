// ADDITIONAL_COMPILE_FLAGS: -std=c++20
#include <cassert>
#include <utility>

enum Overload {
  kLvalueOverload = 1,
  kRvalueOverload = 2,
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

template <class T> Overload forward_by_decltype(T &&x) {
  return chosen_overload(std::forward<decltype(x)>(x));
}

static Overload forward_abbreviated(auto &&x) {
  return chosen_overload(std::forward<decltype(x)>(x));
}

static int forward_abbreviated_pack(auto &&...args) {
  return (chosen_overload(std::forward<decltype(args)>(args)) + ...);
}

int main() {
  Tracked a(3);
  assert(forward_by_decltype(a) == kLvalueOverload);
  assert(a.v == 3);
  assert(forward_by_decltype(Tracked(4)) == kRvalueOverload);

  Tracked b(5);
  assert(forward_abbreviated(b) == kLvalueOverload);
  assert(b.v == 5);
  assert(forward_abbreviated(Tracked(6)) == kRvalueOverload);

  Tracked c(7);
  assert(forward_abbreviated_pack(c, Tracked(8)) ==
         kLvalueOverload + kRvalueOverload);
  assert(c.v == 7);

  return 0;
}
