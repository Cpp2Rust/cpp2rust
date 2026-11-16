// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.mptr.oper]/6: the result of .* with a pointer to data member is
// an lvalue if the first operand is an lvalue and an xvalue otherwise.
#include <cassert>
#include <utility>

struct CountsCopiesAndMoves {
  static inline int copyCount = 0;
  static inline int moveCount = 0;

  CountsCopiesAndMoves() = default;
  CountsCopiesAndMoves(const CountsCopiesAndMoves &) { ++copyCount; }
  CountsCopiesAndMoves(CountsCopiesAndMoves &&) { ++moveCount; }
};

struct HoldsCounted {
  CountsCopiesAndMoves member;
};

int main() {
  CountsCopiesAndMoves HoldsCounted::*pm = &HoldsCounted::member;
  HoldsCounted holder;
  CountsCopiesAndMoves from_lvalue = holder.*pm;
  assert(CountsCopiesAndMoves::copyCount == 1);
  assert(CountsCopiesAndMoves::moveCount == 0);
  CountsCopiesAndMoves from_xvalue = std::move(holder).*pm;
  assert(CountsCopiesAndMoves::copyCount == 1);
  assert(CountsCopiesAndMoves::moveCount == 1);
  return 0;
}
