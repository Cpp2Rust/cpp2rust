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

// One decimal digit per argument, left to right: forward_pack(a, Tracked(4),
// i, 5) == 1234 means the four arguments picked kLvalueOverload,
// kRvalueOverload, kIntLvalueOverload and kIntRvalueOverload.
template <class... Args> int forward_pack(Args &&...args) {
  int digits = 0;
  ((digits = digits * 10 + chosen_overload(std::forward<Args>(args))), ...);
  return digits;
}

struct Pair {
  Tracked a;
  Tracked b;
  template <class A, class B>
  Pair(A &&x, B &&y) : a(std::forward<A>(x)), b(std::forward<B>(y)) {}
};

template <class... Args> Pair forward_pack_into_ctor(Args &&...args) {
  return Pair(std::forward<Args>(args)...);
}

int main() {
  assert(forward_pack() == 0);

  Tracked a(1);
  assert(forward_pack(a) == kLvalueOverload);
  assert(a.v == 1);
  assert(forward_pack(Tracked(2)) == kRvalueOverload);

  int i = 3;
  assert(forward_pack(a, Tracked(4), i, 5) == 1234);
  assert(a.v == 1);
  assert(i == 3);

  Tracked lhs(6);
  Pair p = forward_pack_into_ctor(lhs, Tracked(7));
  assert(p.a.v == 6);
  assert(p.a.copies == 1);
  assert(p.b.v == 7);
  assert(p.b.copies == 0);
  assert(p.b.moves == 1);
  assert(lhs.v == 6);

  return 0;
}
