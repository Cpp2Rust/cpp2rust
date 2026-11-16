// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [basic.scope.namespace]/1: for each non-friend redeclaration whose
// target scope is the namespace scope, the portion after the declarator-id is
// also included in the scope.
namespace p1_block_declaration_in_qualified_definition {
static int global_calls;
static int inner_calls;

int helper() {
  ++global_calls;
  return 100;
}

namespace Outer {
namespace Inner {
int run();
}

int Inner::run() {
  int helper();
  return helper();
}

namespace Inner {
int helper() {
  ++inner_calls;
  return 9;
}
} // namespace Inner
} // namespace Outer

void test() {
  assert(Outer::Inner::run() == 9);
  assert(inner_calls == 1);
  assert(global_calls == 0);
}
} // namespace p1_block_declaration_in_qualified_definition

// N4950 [basic.scope.namespace]/1: for each non-friend redeclaration whose
// target scope is the namespace scope, the portion after the declarator-id is
// also included in the scope.
namespace p1_qualified_definition_is_in_namespace_scope {
static int global_calls;
static int inner_calls;

int base() {
  ++global_calls;
  return 100;
}

namespace Outer {
namespace Inner {
int base() {
  ++inner_calls;
  return 7;
}
int twice();
extern int stored;
} // namespace Inner

int Inner::twice() { return base() * 2; }
int Inner::stored = base() + 1;
} // namespace Outer

void test() {
  assert(Outer::Inner::stored == 8);
  assert(inner_calls == 1);

  assert(Outer::Inner::twice() == 14);
  assert(inner_calls == 2);
  assert(global_calls == 0);
}
} // namespace p1_qualified_definition_is_in_namespace_scope

// N4950 [basic.scope.namespace]/1: any namespace-definition for a namespace N
// introduces a namespace scope that includes the namespace-body for every
// namespace-definition for N.
namespace p1_reopened_namespace_shares_scope {
namespace Tally {
int count = 4;
}

int count = 100;

namespace Tally {
int next() { return ++count; }
} // namespace Tally

void test() {
  assert(Tally::next() == 5);
  assert(Tally::count == 5);
  assert(count == 100);
  assert(p1_reopened_namespace_shares_scope::count == 100);
}
} // namespace p1_reopened_namespace_shares_scope

int main() {
  p1_block_declaration_in_qualified_definition::test();
  p1_qualified_definition_is_in_namespace_scope::test();
  p1_reopened_namespace_shares_scope::test();
  return 0;
}
