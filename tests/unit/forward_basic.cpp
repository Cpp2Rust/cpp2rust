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

struct Holder {
  Tracked t;
  template <class T> Holder(T &&x) : t(std::forward<T>(x)) {}
};

template <class T> Overload forward_once(T &&x) {
  return chosen_overload(std::forward<T>(x));
}

template <class T> Overload forward_twice(T &&x) {
  return forward_once(std::forward<T>(x));
}

template <class T> Holder forward_into_ctor(T &&x) {
  return Holder(std::forward<T>(x));
}

int main() {
  Tracked lvalue(7);
  assert(forward_once(lvalue) == kLvalueOverload);
  assert(lvalue.v == 7);
  assert(forward_once(Tracked(8)) == kRvalueOverload);

  Tracked relayed(9);
  assert(forward_twice(relayed) == kLvalueOverload);
  assert(relayed.v == 9);
  assert(forward_twice(Tracked(10)) == kRvalueOverload);

  Tracked kept(11);
  Holder from_lvalue = forward_into_ctor(kept);
  assert(from_lvalue.t.v == 11);
  assert(from_lvalue.t.copies == 1);
  assert(from_lvalue.t.moves == 0);
  assert(kept.v == 11);

  Holder from_rvalue = forward_into_ctor(Tracked(12));
  assert(from_rvalue.t.v == 12);
  assert(from_rvalue.t.copies == 0);
  assert(from_rvalue.t.moves == 1);

  return 0;
}
