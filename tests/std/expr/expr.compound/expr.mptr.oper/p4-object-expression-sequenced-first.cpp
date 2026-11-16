// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.mptr.oper]/4: in E1.*E2, E1 is sequenced before E2.
#include <cassert>

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

int main() {
  assert(select_object().*select_member() == 7);
  assert(object_stamp == 1);
  assert(member_stamp == 2);
  return 0;
}
