// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <algorithm>
#include <cassert>
#include <vector>

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

// N4950 [dcl.fct.default]/9: "A default argument is evaluated each time the
// function is called with no argument for the corresponding parameter."
namespace p9_evaluated_each_time_called {
static int defaults_evaluated;

int count_default() { return ++defaults_evaluated; }

int combine(int first, int second = count_default()) {
  return first * 100 + second;
}

struct Widget {
  int id;

  Widget(int id = count_default()) : id(id) {}
};

class Account {
  static inline int base = 10;

  enum { kBonus = 5 };

  static int twice(int value) { return 2 * value; }

public:
  int balance = 0;

  int deposit(int amount = twice(base) + kBonus) {
    balance += amount;
    return balance;
  }

  static void set_base(int value) { base = value; }
};

void test() {
  defaults_evaluated = 0;
  assert(combine(1) == 101);
  assert(combine(2) == 202);
  assert(combine(3, 9) == 309);
  assert(defaults_evaluated == 2);

  int first = 0;
  assert(combine(++first) == 103);
  assert(first == 1);
  assert(defaults_evaluated == 3);

  Widget a;
  Widget b(40);
  Widget c;
  assert(a.id == 4);
  assert(b.id == 40);
  assert(c.id == 5);
  assert(defaults_evaluated == 5);

  Account account;
  assert(account.deposit() == 25);
  Account::set_base(1);
  assert(account.deposit() == 32);
  assert(account.deposit(3) == 35);
}
} // namespace p9_evaluated_each_time_called

// N4950 [dcl.fct.default]/9: "A default argument is not part of the type of a
// function."
namespace p9_not_part_of_the_type {
int add(int a, int b = 2) { return a + b; }

struct Scale {
  static int times(int a, int b = 3) { return a * b; }
};

bool descending(const int &a, const int &b = 0) { return a > b; }

void test() {
  int (*p)(int, int) = add;
  int (*q)(int, int) = &add;
  assert(p == q);
  assert(p(1, 5) == 6);
  assert(q(2, 5) == 7);
  assert(add(1) == 3);

  int (*s)(int, int) = &Scale::times;
  assert(s(2, 5) == 10);
  assert(Scale::times(2) == 6);

  std::vector<int> values = {3, -1, 4, 1, 5};
  std::stable_sort(values.begin(), values.end(), descending);
  assert(values[0] == 5);
  assert(values[1] == 4);
  assert(values[2] == 3);
  assert(values[3] == 1);
  assert(values[4] == -1);
  assert(descending(1));
  assert(!descending(-1));
}
} // namespace p9_not_part_of_the_type

int main() {
  p5_names_looked_up_at_declaration::test();
  p9_evaluated_on_each_call::test();
  p9_static_member_hides_namespace_variable::test();
  p9_evaluated_each_time_called::test();
  p9_not_part_of_the_type::test();
  return 0;
}
