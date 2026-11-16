// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <cstring>

// N4950 [basic.scope.pdecl]/1: the locus of a declaration that is a declarator
// is immediately after the complete declarator. /2: a name from an outer scope
// remains visible up to the locus of the declaration that hides it.
namespace p1_earlier_declarator_reads_outer {
void test() {
  int level = 5;
  {
    int saved = level, level = saved + 1, next = level + 1;
    assert(saved == 5);
    assert(level == 6);
    assert(next == 7);
  }
  assert(level == 5);
}
} // namespace p1_earlier_declarator_reads_outer

// N4950 [basic.scope.pdecl]/1: the locus of a declaration that is a declarator
// is immediately after the complete declarator, so the initializer names the
// entity being declared.
namespace p1_own_address_in_initializer {
struct SelfLinked {
  SelfLinked *next;
  int value;
};

void test() {
  SelfLinked node{nullptr, 3};
  SelfLinked *outer = &node;
  {
    SelfLinked node{&node, 7};
    assert(node.next == &node);
    assert(node.next != outer);
    assert(node.next->value == 7);
  }
  assert(node.next == nullptr);
  assert(node.value == 3);
}
} // namespace p1_own_address_in_initializer

// N4950 [basic.scope.pdecl]/2: a name from an outer scope remains visible up
// to the locus of the declaration that hides it.
namespace p2_array_bound_names_outer {
void test() {
  const int length = 2;
  {
    int length[length] = {7, 8};
    assert(sizeof(length) / sizeof(length[0]) == 2);
    assert(length[0] == 7);
    assert(length[1] == 8);
  }
  assert(length == 2);
}
} // namespace p2_array_bound_names_outer

// N4950 [basic.scope.pdecl]/5: the locus of an enumerator-definition is
// immediately after it.
namespace p5_enumerator_initializer_names_outer {
void test() {
  const int limit = 12;
  {
    enum { limit = limit + 1, above = limit + 1 };
    assert(limit == 13);
    assert(above == 14);
  }
  assert(limit == 12);
}
} // namespace p5_enumerator_initializer_names_outer

// N4950 [basic.scope.pdecl]/9: the locus of the implicit declaration of a
// function-local predefined variable is immediately before the function-body
// of its function's definition.
namespace p9_func_declared_before_function_body {
struct RecordsConstructorName {
  const char *name;

  RecordsConstructorName() : name(__func__) { assert(name == __func__); }
};

const char *plain_function() { return __func__; }

void test() {
  assert(std::strcmp(plain_function(), "plain_function") == 0);
  assert(plain_function() == plain_function());

  RecordsConstructorName object;
  assert(std::strcmp(object.name, "RecordsConstructorName") == 0);

  const char *enclosing = __func__;
  auto lambda = [] { return __func__; };
  assert(lambda() != enclosing);
  assert(std::strcmp(enclosing, "test") == 0);
}
} // namespace p9_func_declared_before_function_body

// N4950 [basic.scope.pdecl]/11: the locus of a for-range-declaration of a
// range-based for statement is immediately after the for-range-initializer.
namespace p11_range_initializer_names_outer {
void test() {
  int values[3] = {2, 5, 9};
  int steps = 0;
  int sum = 0;
  for (int values : values) {
    values *= 2;
    ++steps;
    sum += values;
  }
  assert(steps == 3);
  assert(sum == 32);
  assert(values[0] == 2);
  assert(values[1] == 5);
  assert(values[2] == 9);
}
} // namespace p11_range_initializer_names_outer

int main() {
  p1_earlier_declarator_reads_outer::test();
  p1_own_address_in_initializer::test();
  p2_array_bound_names_outer::test();
  p5_enumerator_initializer_names_outer::test();
  p9_func_declared_before_function_body::test();
  p11_range_initializer_names_outer::test();
  return 0;
}
