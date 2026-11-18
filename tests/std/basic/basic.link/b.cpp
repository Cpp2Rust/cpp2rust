namespace p3_1_static_functions_in_two_files {
static int pick() { return 2; }

int pick_in_b() { return pick(); }
} // namespace p3_1_static_functions_in_two_files

namespace p3_2_const_variables_in_two_files {
const int limit = 20;

int limit_in_b() { return limit; }
const int *limit_address_in_b() { return &limit; }
} // namespace p3_2_const_variables_in_two_files

namespace p3_2_2_extern_const_variable {
extern const int threshold = 30;

const int *threshold_address_in_b() { return &threshold; }
} // namespace p3_2_2_extern_const_variable

namespace p3_2_4_const_variable_previously_declared_extern {
extern const int ceiling;
const int ceiling = 40;

const int *ceiling_address_in_b() { return &ceiling; }
} // namespace p3_2_4_const_variable_previously_declared_extern

namespace p3_3_anonymous_union_members_in_two_files {
static union {
  int whole;
  float fraction;
};

void set_whole_in_b(int value) { whole = value; }
int whole_in_b() { return whole; }
} // namespace p3_3_anonymous_union_members_in_two_files

namespace p4_unnamed_namespaces_in_two_files {
namespace {
int counter = 50;

int bump() { return ++counter; }

struct Record {
  long id = 200;
  int extra = 1;
};

namespace Nested {
int depth = 4;
}
} // namespace

int bump_in_b() { return bump(); }
int record_id_in_b() {
  Record record;
  return static_cast<int>(record.id);
}
int depth_in_b() { return Nested::depth; }
} // namespace p4_unnamed_namespaces_in_two_files

namespace p4_1_variable_defined_in_other_file {
int shared_total = 5;

void add_in_b(int amount) { shared_total += amount; }
} // namespace p4_1_variable_defined_in_other_file

namespace p4_9_members_of_named_namespace {
namespace Config {
int level = 1;

int raise(int by) {
  level += by;
  return level;
}
} // namespace Config
} // namespace p4_9_members_of_named_namespace

namespace p5_members_of_classes_in_unnamed_namespaces {
namespace {
struct Tally {
  static int total;

  int add(int amount) {
    total += amount * 2;
    return total;
  }
};
int Tally::total = 100;
} // namespace

int add_in_b(int amount) {
  Tally tally;
  return tally.add(amount);
}
} // namespace p5_members_of_classes_in_unnamed_namespaces

namespace p5_members_of_class_with_external_linkage {
struct Ledger {
  static int balance;

  static int deposit(int amount);
};

int Ledger::balance = 50;

int Ledger::deposit(int amount) {
  balance += amount;
  return balance;
}
} // namespace p5_members_of_class_with_external_linkage

namespace p6_block_scope_extern_names_entity_in_other_file {
int remote = 70;

void set_remote_in_b(int value) { remote = value; }
} // namespace p6_block_scope_extern_names_entity_in_other_file

namespace p6_block_scope_extern_names_hidden_static {
static int hidden = 90;

int hidden_in_b() { return hidden; }
} // namespace p6_block_scope_extern_names_hidden_static

namespace p7_block_scope_static_variables {
int next_in_b() {
  static int count = 0;
  count += 1000;
  return count;
}
} // namespace p7_block_scope_static_variables

namespace p7_local_classes {
int key_in_b() {
  struct Entry {
    long key = 400;
  };
  Entry entry;
  return static_cast<int>(entry.key);
}
} // namespace p7_local_classes

namespace p11_array_declared_without_bound {
int table[3] = {1, 2, 3};

int table_sum_in_b() { return table[0] + table[1] + table[2]; }
} // namespace p11_array_declared_without_bound
