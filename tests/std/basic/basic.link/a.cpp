#include <cassert>

// N4950 [basic.link]/3.1: the name of a function that is explicitly declared
// static has internal linkage.
namespace p3_1_static_functions_in_two_files {
static int pick() { return 1; }

int pick_in_b();

void test() {
  assert(pick() == 1);
  assert(pick_in_b() == 2);
}
} // namespace p3_1_static_functions_in_two_files

// N4950 [basic.link]/3.2: the name of a non-template variable of non-volatile
// const-qualified type has internal linkage.
namespace p3_2_const_variables_in_two_files {
const int limit = 10;

int limit_in_b();
const int *limit_address_in_b();

void test() {
  assert(limit == 10);
  assert(limit_in_b() == 20);
  assert(&limit != limit_address_in_b());
}
} // namespace p3_2_const_variables_in_two_files

// N4950 [basic.link]/3.2.2: unless it is explicitly declared extern.
namespace p3_2_2_extern_const_variable {
extern const int threshold;

const int *threshold_address_in_b();

void test() {
  assert(threshold == 30);
  assert(&threshold == threshold_address_in_b());
}
} // namespace p3_2_2_extern_const_variable

// N4950 [basic.link]/3.2.4: or it was previously declared and the prior
// declaration did not have internal linkage.
namespace p3_2_4_const_variable_previously_declared_extern {
extern const int ceiling;

const int *ceiling_address_in_b();

void test() {
  assert(ceiling == 40);
  assert(&ceiling == ceiling_address_in_b());
}
} // namespace p3_2_4_const_variable_previously_declared_extern

// N4950 [basic.link]/3.3: the name of a data member of an anonymous union has
// internal linkage.
namespace p3_3_anonymous_union_members_in_two_files {
static union {
  int whole;
  float fraction;
};

void set_whole_in_b(int value);
int whole_in_b();

void test() {
  whole = 5;
  set_whole_in_b(6);
  assert(whole == 5);
  assert(whole_in_b() == 6);
}
} // namespace p3_3_anonymous_union_members_in_two_files

// N4950 [basic.link]/4: an unnamed namespace or a namespace declared directly
// or indirectly within an unnamed namespace has internal linkage, and so do
// the names of the entities that belong to it.
namespace p4_unnamed_namespaces_in_two_files {
namespace {
int counter = 1;

int bump() { return ++counter; }

struct Record {
  int id = 100;
};

namespace Nested {
int depth = 3;
}
} // namespace

int bump_in_b();
int record_id_in_b();
int depth_in_b();

void test() {
  assert(bump() == 2);
  assert(bump_in_b() == 51);
  assert(counter == 2);

  Record record;
  assert(record.id == 100);
  assert(record_id_in_b() == 200);

  assert(Nested::depth == 3);
  assert(depth_in_b() == 4);
}
} // namespace p4_unnamed_namespaces_in_two_files

// N4950 [basic.link]/4.1: otherwise the name of a variable that belongs to a
// namespace scope has external linkage.
namespace p4_1_variable_defined_in_other_file {
extern int shared_total;

void add_in_b(int amount);

void test() {
  assert(shared_total == 5);
  shared_total = 7;
  add_in_b(3);
  assert(shared_total == 10);
}
} // namespace p4_1_variable_defined_in_other_file

// N4950 [basic.link]/4.9: the name of an entity that belongs to a namespace
// with external linkage has external linkage.
namespace p4_9_members_of_named_namespace {
namespace Config {
extern int level;

int raise(int by);
} // namespace Config

void test() {
  assert(Config::level == 1);
  assert(Config::raise(2) == 3);
  Config::level = 10;
  assert(Config::raise(1) == 11);
}
} // namespace p4_9_members_of_named_namespace

// N4950 [basic.link]/5: a member function or static data member has the same
// linkage as the name of the class of which it is a member.
namespace p5_members_of_classes_in_unnamed_namespaces {
namespace {
struct Tally {
  static int total;

  int add(int amount) {
    total += amount;
    return total;
  }
};
int Tally::total = 10;
} // namespace

int add_in_b(int amount);

void test() {
  Tally tally;
  assert(tally.add(1) == 11);
  assert(add_in_b(1) == 102);
  assert(Tally::total == 11);
}
} // namespace p5_members_of_classes_in_unnamed_namespaces

namespace p5_members_of_class_with_external_linkage {
struct Ledger {
  static int balance;

  static int deposit(int amount);
};

void test() {
  assert(Ledger::balance == 50);
  Ledger::balance = 60;
  assert(Ledger::deposit(5) == 65);
  assert(Ledger::balance == 65);
}
} // namespace p5_members_of_class_with_external_linkage

// N4950 [basic.link]/6: a block-scope extern declaration redeclares the
// entity of the enclosing namespace, with that entity's linkage.
namespace p6_block_scope_extern_names_entity_in_other_file {
int read_remote() {
  extern int remote;
  return remote;
}

void test() {
  extern void set_remote_in_b(int value);
  assert(read_remote() == 70);
  set_remote_in_b(71);
  assert(read_remote() == 71);
}
} // namespace p6_block_scope_extern_names_entity_in_other_file

namespace p6_block_scope_extern_names_hidden_static {
static int hidden = 80;

int hidden_in_b();

int read_hidden_through_extern() {
  int hidden = 1;
  {
    extern int hidden;
    return hidden;
  }
}

void test() {
  assert(read_hidden_through_extern() == 80);
  hidden = 81;
  assert(read_hidden_through_extern() == 81);
  assert(hidden_in_b() == 90);
}
} // namespace p6_block_scope_extern_names_hidden_static

// N4950 [basic.link]/7: a name declared at block scope has no linkage.
namespace p7_block_scope_static_variables {
int next_even() {
  static int count = 0;
  count += 2;
  return count;
}

int next_odd() {
  static int count = 1;
  count += 2;
  return count;
}

int next_in_b();

void test() {
  assert(next_even() == 2);
  assert(next_odd() == 3);
  assert(next_in_b() == 1000);
  assert(next_even() == 4);
  assert(next_odd() == 5);
  assert(next_in_b() == 2000);
}
} // namespace p7_block_scope_static_variables

namespace p7_local_classes {
int first_key() {
  struct Entry {
    int key = 1;
  };
  Entry entry;
  return entry.key;
}

int second_key_plus_value() {
  struct Entry {
    int key = 2;
    int value = 30;
  };
  Entry entry;
  return entry.key + entry.value;
}

int key_in_b();

void test() {
  assert(first_key() == 1);
  assert(second_key_plus_value() == 32);
  assert(key_in_b() == 400);
}
} // namespace p7_local_classes

// N4950 [basic.link]/11: declarations for an array object can specify array
// types that differ by the presence or absence of a major array bound.
namespace p11_array_declared_without_bound {
extern int table[];

int table_sum_in_b();

void test() {
  assert(table[1] == 2);
  table[2] = 30;
  assert(table_sum_in_b() == 33);
}
} // namespace p11_array_declared_without_bound

int main() {
  p3_1_static_functions_in_two_files::test();
  p3_2_const_variables_in_two_files::test();
  p3_2_2_extern_const_variable::test();
  p3_2_4_const_variable_previously_declared_extern::test();
  p3_3_anonymous_union_members_in_two_files::test();
  p4_unnamed_namespaces_in_two_files::test();
  p4_1_variable_defined_in_other_file::test();
  p4_9_members_of_named_namespace::test();
  p5_members_of_classes_in_unnamed_namespaces::test();
  p5_members_of_class_with_external_linkage::test();
  p6_block_scope_extern_names_entity_in_other_file::test();
  p6_block_scope_extern_names_hidden_static::test();
  p7_block_scope_static_variables::test();
  p7_local_classes::test();
  p11_array_declared_without_bound::test();
  return 0;
}
