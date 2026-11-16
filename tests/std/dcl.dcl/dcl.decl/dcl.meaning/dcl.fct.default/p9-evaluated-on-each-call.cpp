// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [dcl.fct.default]/9: a default argument is evaluated each time the
// function is called with no argument for the corresponding parameter.
#include <cassert>

static int next_ticket_calls;

int next_ticket() { return ++next_ticket_calls * 100; }

int issue(int ticket = next_ticket()) { return ticket; }

int main() {
  assert(next_ticket_calls == 0);

  assert(issue() == 100);
  assert(next_ticket_calls == 1);

  assert(issue(7) == 7);
  assert(next_ticket_calls == 1);

  assert(issue() == 200);
  assert(next_ticket_calls == 2);
  return 0;
}
