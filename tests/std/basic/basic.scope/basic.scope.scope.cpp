// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <utility>

// N4950 [basic.scope.scope]/2.10: a block-scope extern declaration targets a
// larger enclosing scope but binds its name in its immediate scope.
namespace p2_10_block_scope_extern_names_namespace_variable {
int level = 11;

void test() {
  int level = 22;
  assert(level == 22);
  {
    extern int level;
    assert(level == 11);
    level = 33;
  }
  assert(level == 22);
  assert(p2_10_block_scope_extern_names_namespace_variable::level == 33);
}
} // namespace p2_10_block_scope_extern_names_namespace_variable

// N4950 [basic.scope.scope]/2.10: a block-scope function declaration targets
// the enclosing namespace scope but binds its name in its immediate scope.
namespace p2_10_block_scope_function_declaration_names_namespace_function {
static int function_calls;

void test() {
  int measure = 5;
  {
    int measure();
    assert(measure() == 7);
    assert(function_calls == 1);
  }
  assert(measure == 5);
}

int measure() {
  ++function_calls;
  return 7;
}
} // namespace p2_10_block_scope_function_declaration_names_namespace_function

// N4950 [basic.scope.scope]/2.11: the name of an unscoped enumerator is bound
// in the enumeration scope and in the scope enclosing the enumeration.
namespace p2_11_unscoped_enumerator_in_both_scopes {
enum Direction { north = 4, south = 9 };

struct HasUnscopedEnum {
  enum Size { small = 3, large = 8 };

  int scale(int count) const { return count * large; }
};

void test() {
  assert(north == 4);
  assert(Direction::north == 4);
  assert(south == 9);
  assert(Direction::south == 9);

  assert(HasUnscopedEnum::small == 3);
  assert(HasUnscopedEnum::Size::small == 3);
  assert(HasUnscopedEnum::large == 8);
  assert(HasUnscopedEnum::Size::large == 8);

  HasUnscopedEnum object;
  assert(object.scale(2) == 16);
  assert(object.small == 3);
}
} // namespace p2_11_unscoped_enumerator_in_both_scopes

// N4950 [basic.scope.scope]/2.12: a class's name is also bound in its own
// scope.
namespace p2_12_class_name_in_own_scope {
struct HasStaticCounter {
  static inline int count = 0;

  static void bump() { ++HasStaticCounter::count; }
};

void test() {
  HasStaticCounter::bump();
  assert(HasStaticCounter::count == 1);

  HasStaticCounter::HasStaticCounter::bump();
  assert(HasStaticCounter::count == 2);

  HasStaticCounter::HasStaticCounter::HasStaticCounter::count = 6;
  assert(HasStaticCounter::count == 6);
}
} // namespace p2_12_class_name_in_own_scope

// N4950 [basic.scope.scope]/2.13: the names of the members of an anonymous
// union are bound in the union's parent scope.
namespace p2_13_anonymous_union_members_in_parent_scope {
static union {
  int namespace_count;
  double namespace_ratio;
};

struct HasAnonymousUnion {
  int tag;
  union {
    int member_count;
    double member_ratio;
  };

  int read_count() const { return member_count; }
};

void test() {
  namespace_count = 4;
  assert(namespace_count == 4);
  namespace_ratio = 0.25;
  assert(namespace_ratio == 0.25);

  HasAnonymousUnion object;
  object.tag = 1;
  object.member_count = 6;
  assert(object.member_count == 6);
  assert(object.read_count() == 6);
  object.member_ratio = 0.5;
  assert(object.member_ratio == 0.5);
  assert(object.tag == 1);

  union {
    int block_count;
    double block_ratio;
  };
  block_count = 9;
  assert(block_count == 9);
  block_ratio = 0.75;
  assert(block_ratio == 0.75);
}
} // namespace p2_13_anonymous_union_members_in_parent_scope

// N4950 [basic.scope.scope]/3.2: two non-static member functions have
// corresponding object parameters if their object parameters have the same
// type. /4.3.1: functions whose object parameters do not correspond do not
// correspond, so both are declared in the class.
namespace p3_2_const_qualified_member_is_distinct {
struct HasConstOverload {
  int plain_calls = 0;
  mutable int const_calls = 0;

  int get() {
    ++plain_calls;
    return 1;
  }
  int get() const {
    ++const_calls;
    return 2;
  }
};

void test() {
  HasConstOverload object;
  const HasConstOverload &view = object;

  assert(object.get() == 1);
  assert(object.plain_calls == 1);
  assert(object.const_calls == 0);

  assert(view.get() == 2);
  assert(object.plain_calls == 1);
  assert(object.const_calls == 1);
}
} // namespace p3_2_const_qualified_member_is_distinct

// N4950 [basic.scope.scope]/3.2: two non-static member functions have
// corresponding object parameters if their object parameters have the same
// type. An explicit object parameter of type X& and the implicit object
// parameter of a &&-qualified member have different types.
namespace p3_2_explicit_object_and_rvalue_members_are_distinct {
struct HasExplicitObjectOverload {
  static inline int explicit_calls = 0;
  static inline int rvalue_calls = 0;

  int base = 10;

  int pick(this HasExplicitObjectOverload &self, int amount) {
    ++explicit_calls;
    return self.base + amount;
  }
  int pick(int amount) && {
    ++rvalue_calls;
    return base + amount + 100;
  }
};

void test() {
  HasExplicitObjectOverload object;

  assert(object.pick(1) == 11);
  assert(HasExplicitObjectOverload::explicit_calls == 1);
  assert(HasExplicitObjectOverload::rvalue_calls == 0);

  assert(HasExplicitObjectOverload().pick(2) == 112);
  assert(HasExplicitObjectOverload::explicit_calls == 1);
  assert(HasExplicitObjectOverload::rvalue_calls == 1);
}
} // namespace p3_2_explicit_object_and_rvalue_members_are_distinct

// N4950 [basic.scope.scope]/3.2: two non-static member functions have
// corresponding object parameters if their object parameters have the same
// type. /4.3.1: functions whose object parameters do not correspond do not
// correspond, so both are declared in the class.
namespace p3_2_ref_qualified_members_are_distinct {
struct HasRefQualifiedOverloads {
  static inline int lvalue_calls = 0;
  static inline int rvalue_calls = 0;

  int get() & {
    ++lvalue_calls;
    return 3;
  }
  int get() && {
    ++rvalue_calls;
    return 4;
  }
};

void test() {
  HasRefQualifiedOverloads object;

  assert(object.get() == 3);
  assert(HasRefQualifiedOverloads::lvalue_calls == 1);
  assert(HasRefQualifiedOverloads::rvalue_calls == 0);

  assert(HasRefQualifiedOverloads().get() == 4);
  assert(HasRefQualifiedOverloads::lvalue_calls == 1);
  assert(HasRefQualifiedOverloads::rvalue_calls == 1);

  assert(std::move(object).get() == 4);
  assert(HasRefQualifiedOverloads::lvalue_calls == 1);
  assert(HasRefQualifiedOverloads::rvalue_calls == 2);
}
} // namespace p3_2_ref_qualified_members_are_distinct

// N4950 [basic.scope.scope]/3.4: two non-static member function templates
// have corresponding object parameters if the types of their object
// parameters are equivalent. /4.3.2: function templates whose signatures do
// not correspond do not correspond, so both are declared in the class.
namespace p3_4_const_qualified_member_templates_are_distinct {
struct HasConstTemplateOverload {
  int plain_calls = 0;
  mutable int const_calls = 0;

  template <class Value> int get(Value value) {
    ++plain_calls;
    return value + 1;
  }
  template <class Value> int get(Value value) const {
    ++const_calls;
    return value + 2;
  }
};

void test() {
  HasConstTemplateOverload object;
  const HasConstTemplateOverload &view = object;

  assert(object.get(10) == 11);
  assert(object.plain_calls == 1);
  assert(object.const_calls == 0);

  assert(view.get(10) == 12);
  assert(object.plain_calls == 1);
  assert(object.const_calls == 1);
}
} // namespace p3_4_const_qualified_member_templates_are_distinct

// N4950 [basic.scope.scope]/3.4: two non-static member function templates
// have corresponding object parameters if the types of their object
// parameters are equivalent. /4.3.2: function templates whose signatures do
// not correspond do not correspond, so both are declared in the class.
namespace p3_4_ref_qualified_member_templates_are_distinct {
struct HasRefQualifiedTemplateOverloads {
  static inline int lvalue_calls = 0;
  static inline int rvalue_calls = 0;

  template <class Value> int get(Value value) & {
    ++lvalue_calls;
    return value + 3;
  }
  template <class Value> int get(Value value) && {
    ++rvalue_calls;
    return value + 4;
  }
};

void test() {
  HasRefQualifiedTemplateOverloads object;

  assert(object.get(10) == 13);
  assert(HasRefQualifiedTemplateOverloads::lvalue_calls == 1);
  assert(HasRefQualifiedTemplateOverloads::rvalue_calls == 0);

  assert(HasRefQualifiedTemplateOverloads().get(10) == 14);
  assert(HasRefQualifiedTemplateOverloads::lvalue_calls == 1);
  assert(HasRefQualifiedTemplateOverloads::rvalue_calls == 1);

  assert(std::move(object).get(10) == 14);
  assert(HasRefQualifiedTemplateOverloads::lvalue_calls == 1);
  assert(HasRefQualifiedTemplateOverloads::rvalue_calls == 2);
}
} // namespace p3_4_ref_qualified_member_templates_are_distinct

// N4950 [basic.scope.scope]/4: two declarations correspond if they
// (re)introduce the same name; they can correspond even if neither binds a
// name.
namespace p4_friend_definition_defines_earlier_friend_declaration {
static int friend_calls;

struct DeclaresFriend {
  friend int shared_friend();
};

struct DefinesFriend {
  friend int shared_friend() {
    ++friend_calls;
    return 7;
  }
};

int shared_friend();

void test() {
  assert(shared_friend() == 7);
  assert(friend_calls == 1);
}
} // namespace p4_friend_definition_defines_earlier_friend_declaration

// N4950 [basic.scope.scope]/4.1: two declarations that (re)introduce the same
// name do not correspond if either is a using-declarator.
namespace p4_1_using_declarator_does_not_correspond {
static int library_calls;
static int local_calls;

namespace Library {
int describe(int amount) {
  ++library_calls;
  return amount + 10;
}
} // namespace Library

using Library::describe;

int describe(double amount) {
  ++local_calls;
  return static_cast<int>(amount) + 20;
}

void test() {
  assert(describe(1) == 11);
  assert(library_calls == 1);
  assert(local_calls == 0);

  assert(describe(2.0) == 22);
  assert(library_calls == 1);
  assert(local_calls == 1);
}
} // namespace p4_1_using_declarator_does_not_correspond

// N4950 [basic.scope.scope]/4.2: two declarations that (re)introduce the same
// name do not correspond if one declares a type (not a typedef-name) and the
// other declares a function.
namespace p4_2_class_and_function_share_name {
static int function_calls;

struct Reading {
  int value;
};

int Reading(int raw) {
  ++function_calls;
  return raw + 1;
}

void test() {
  struct Reading reading = {4};
  assert(reading.value == 4);
  assert(function_calls == 0);

  assert(Reading(reading.value) == 5);
  assert(function_calls == 1);
}
} // namespace p4_2_class_and_function_share_name

// N4950 [basic.scope.scope]/4.2: two declarations that (re)introduce the same
// name do not correspond if one declares a type (not a typedef-name) and the
// other declares a variable.
namespace p4_2_class_and_variable_share_name {
struct Limit {
  int value;
};

int Limit = 12;

void test() {
  struct Limit limit = {3};
  assert(limit.value == 3);
  assert(Limit == 12);

  Limit += limit.value;
  assert(Limit == 15);
  assert(limit.value == 3);
}
} // namespace p4_2_class_and_variable_share_name

// N4950 [basic.scope.scope]/4.3.1: two function declarations correspond when
// they have the same non-object-parameter-type-list; otherwise each declares
// its own function.
namespace p4_3_1_typedef_parameter_redeclares_function {
static int int_calls;
static int unit_calls;

typedef int Count;
enum Unit : int { single = 1 };

int describe(int amount);

int describe(Unit unit) {
  ++unit_calls;
  return unit + 20;
}

void test() {
  assert(describe(3) == 13);
  assert(int_calls == 1);
  assert(unit_calls == 0);

  assert(describe(single) == 21);
  assert(int_calls == 1);
  assert(unit_calls == 1);
}

int describe(Count amount) {
  ++int_calls;
  return amount + 10;
}
} // namespace p4_3_1_typedef_parameter_redeclares_function

// N4950 [basic.scope.scope]/4.3.2: two function template declarations
// correspond when they have corresponding signatures and equivalent
// template-heads; otherwise each declares its own function template.
namespace p4_3_2_equivalent_template_redeclares_function_template {
static int value_calls;
static int pointer_calls;

template <class Value> int describe(Value value);

template <class Value> int describe(Value *pointer) {
  ++pointer_calls;
  return *pointer + 20;
}

void test() {
  int amount = 3;

  assert(describe(amount) == 13);
  assert(value_calls == 1);
  assert(pointer_calls == 0);

  assert(describe(&amount) == 23);
  assert(value_calls == 1);
  assert(pointer_calls == 1);
}

template <class Renamed> int describe(Renamed renamed) {
  ++value_calls;
  return renamed + 10;
}
} // namespace p4_3_2_equivalent_template_redeclares_function_template

// N4950 [basic.scope.scope]/4.3.2: two function template declarations
// correspond when they have corresponding signatures and equivalent
// template-heads; otherwise each declares its own function template.
namespace p4_3_2_function_templates_with_different_parameter_lists {
static int value_calls;
static int pointer_calls;

template <class Value> int describe(Value value) {
  ++value_calls;
  return value + 10;
}

template <class Value> int describe(Value *pointer) {
  ++pointer_calls;
  return *pointer + 20;
}

void test() {
  int amount = 3;

  assert(describe(amount) == 13);
  assert(value_calls == 1);
  assert(pointer_calls == 0);

  assert(describe(&amount) == 23);
  assert(value_calls == 1);
  assert(pointer_calls == 1);
}
} // namespace p4_3_2_function_templates_with_different_parameter_lists

// N4950 [basic.scope.scope]/4.3.2: two function template declarations
// correspond when they have corresponding signatures and equivalent
// template-heads; otherwise each declares its own function template.
namespace p4_3_2_function_templates_with_different_template_heads {
static int type_parameter_calls;
static int value_parameter_calls;

template <class Value> int describe(Value value) {
  ++type_parameter_calls;
  return value + 10;
}

template <int amount> int describe() {
  ++value_parameter_calls;
  return amount + 20;
}

void test() {
  assert(describe(3) == 13);
  assert(type_parameter_calls == 1);
  assert(value_parameter_calls == 0);

  assert(describe<3>() == 23);
  assert(type_parameter_calls == 1);
  assert(value_parameter_calls == 1);
}
} // namespace p4_3_2_function_templates_with_different_template_heads

// N4950 [basic.scope.scope]/6: a declaration is nominable in a namespace E if
// its target scope is the scope of E or of any element of the inline namespace
// set of E.
namespace p6_definition_through_inline_namespace {
static int version_calls;

namespace Api {
inline namespace Latest {
int version();
extern int revision;
} // namespace Latest
} // namespace Api

int Api::version() {
  ++version_calls;
  return 2;
}

int Api::revision = 40;

void test() {
  assert(Api::version() == 2);
  assert(version_calls == 1);

  assert(Api::Latest::version() == 2);
  assert(version_calls == 2);

  assert(&Api::revision == &Api::Latest::revision);
  Api::revision = 41;
  assert(Api::Latest::revision == 41);
}
} // namespace p6_definition_through_inline_namespace

// N4950 [basic.scope.scope]/7: a scope introduced by a template definition is
// considered to be introduced by the instantiated entity and to contain the
// instantiations of the declarations that inhabit it.
namespace p7_specialization_has_own_block_variables {
template <class Tag> int next_id() {
  static int last = 0;
  return ++last;
}

void test() {
  assert(next_id<int>() == 1);
  assert(next_id<int>() == 2);
  assert(next_id<char>() == 1);
  assert(next_id<int>() == 3);
  assert(next_id<char>() == 2);
}
} // namespace p7_specialization_has_own_block_variables

int main() {
  p2_10_block_scope_extern_names_namespace_variable::test();
  p2_10_block_scope_function_declaration_names_namespace_function::test();
  p2_11_unscoped_enumerator_in_both_scopes::test();
  p2_12_class_name_in_own_scope::test();
  p2_13_anonymous_union_members_in_parent_scope::test();
  p3_2_const_qualified_member_is_distinct::test();
  p3_2_explicit_object_and_rvalue_members_are_distinct::test();
  p3_2_ref_qualified_members_are_distinct::test();
  p3_4_const_qualified_member_templates_are_distinct::test();
  p3_4_ref_qualified_member_templates_are_distinct::test();
  p4_friend_definition_defines_earlier_friend_declaration::test();
  p4_1_using_declarator_does_not_correspond::test();
  p4_2_class_and_function_share_name::test();
  p4_2_class_and_variable_share_name::test();
  p4_3_1_typedef_parameter_redeclares_function::test();
  p4_3_2_equivalent_template_redeclares_function_template::test();
  p4_3_2_function_templates_with_different_parameter_lists::test();
  p4_3_2_function_templates_with_different_template_heads::test();
  p6_definition_through_inline_namespace::test();
  p7_specialization_has_own_block_variables::test();
  return 0;
}
