// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <utility>

// N4950 [expr.mptr.oper]/3: E1->*E2 is converted into (*(E1)).*E2.
namespace p3_arrow_star {
struct HasTwoInts {
  int first;
  int second;
};

void test() {
  int HasTwoInts::*pm = &HasTwoInts::second;
  HasTwoInts obj{3, 4};
  HasTwoInts *p = &obj;
  assert(p->*pm == 4);
  p->*pm = 5;
  assert(obj.second == 5);
  assert(obj.first == 3);
  assert(&(p->*pm) == &((*p).*pm));
}
} // namespace p3_arrow_star

// N4950 [expr.mptr.oper]/4: in E1.*E2, E1 is sequenced before E2.
namespace p4_object_expression_sequenced_first {
struct HasValue {
  int value;
};

static int sequence;
static int object_stamp;
static int member_stamp;
static HasValue object{7};

static HasValue &select_object() {
  object_stamp = ++sequence;
  return object;
}

static int HasValue::*select_member() {
  member_stamp = ++sequence;
  return &HasValue::value;
}

void test() {
  assert(select_object().*select_member() == 7);
  assert(object_stamp == 1);
  assert(member_stamp == 2);
}
} // namespace p4_object_expression_sequenced_first

// N4950 [expr.mptr.oper]/6: the result of .* with a pointer to data member is
// an lvalue if the first operand is an lvalue.
namespace p6_data_member_of_lvalue_is_lvalue {
struct HasTwoInts {
  int first;
  int second;
};

void test() {
  int HasTwoInts::*pm = &HasTwoInts::second;
  HasTwoInts obj{3, 4};
  assert(&(obj.*pm) == &obj.second);
  int &ref = obj.*pm;
  ref = 5;
  assert(obj.second == 5);
  assert(obj.first == 3);
}
} // namespace p6_data_member_of_lvalue_is_lvalue

// N4950 [expr.mptr.oper]/6: the result of .* with a pointer to data member is
// an lvalue if the first operand is an lvalue and an xvalue otherwise.
namespace p6_data_member_of_rvalue_is_xvalue {
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

void test() {
  CountsCopiesAndMoves HoldsCounted::*pm = &HoldsCounted::member;
  HoldsCounted holder;
  CountsCopiesAndMoves from_lvalue = holder.*pm;
  assert(CountsCopiesAndMoves::copyCount == 1);
  assert(CountsCopiesAndMoves::moveCount == 0);
  CountsCopiesAndMoves from_xvalue = std::move(holder).*pm;
  assert(CountsCopiesAndMoves::copyCount == 1);
  assert(CountsCopiesAndMoves::moveCount == 1);
}
} // namespace p6_data_member_of_rvalue_is_xvalue

int main() {
  p3_arrow_star::test();
  p4_object_expression_sequenced_first::test();
  p6_data_member_of_lvalue_is_lvalue::test();
  p6_data_member_of_rvalue_is_xvalue::test();
  return 0;
}
