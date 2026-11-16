// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [dcl.fct.default]/5: the names in the default argument are looked up
// at the point where the default argument appears.
namespace p5_names_looked_up_at_declaration {
static int scaled_calls;

int rate = 1;

int scaled(int amount) {
  ++scaled_calls;
  return amount * 10;
}

int total(int base = scaled(rate)) { return base; }

void test() {
  rate = 2;
  {
    int rate = 3;
    assert(total() == 20);
    assert(scaled_calls == 1);
    assert(rate == 3);
  }
  assert(total(5) == 5);
  assert(scaled_calls == 1);
}
} // namespace p5_names_looked_up_at_declaration

// N4950 [dcl.fct.default]/9: a default argument is evaluated each time the
// function is called with no argument for the corresponding parameter.
namespace p9_evaluated_on_each_call {
static int next_ticket_calls;

int next_ticket() { return ++next_ticket_calls * 100; }

int issue(int ticket = next_ticket()) { return ticket; }

void test() {
  assert(next_ticket_calls == 0);

  assert(issue() == 100);
  assert(next_ticket_calls == 1);

  assert(issue(7) == 7);
  assert(next_ticket_calls == 1);

  assert(issue() == 200);
  assert(next_ticket_calls == 2);
}
} // namespace p9_evaluated_on_each_call

// N4950 [dcl.fct.default]/9: a non-static member shall not appear in a default
// argument; a static member needs no object and can be named there.
namespace p9_static_member_hides_namespace_variable {
int step = 100;

struct HasStaticStep {
  static inline int step = 4;

  int total = 1;

  int advance(int amount = step) {
    total += amount;
    return total;
  }
};

void test() {
  HasStaticStep object;
  assert(object.advance() == 5);

  HasStaticStep::step = 6;
  assert(object.advance() == 11);
  assert(object.advance(2) == 13);
  assert(step == 100);
}
} // namespace p9_static_member_hides_namespace_variable

int main() {
  p5_names_looked_up_at_declaration::test();
  p9_evaluated_on_each_call::test();
  p9_static_member_hides_namespace_variable::test();
  return 0;
}
