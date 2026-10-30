#include <cassert>
#include <utility>

struct Celsius {
  int degrees;
  Celsius(int d) : degrees(d) {}
};

struct Meters {
  int value;
  operator int() const { return value; }
};

struct Label {
  int id;
  Label(const Celsius &c) : id(c.degrees) {}
};

struct Owner {
  int taken;
  int copied;
  Owner(const Celsius &c) : taken(0), copied(c.degrees) {}
  Owner(Celsius &&c) : taken(c.degrees), copied(0) {}
};

struct Holder {
  int value;
  Holder(Celsius c) : value(c.degrees) {}
};

struct Fallback {
  int value;
  Fallback(int v = -1) : value(v) {}
};

struct Counter {
  int hits;
};

struct Observer {
  int seen;
  Observer(Counter &c) : seen(c.hits) { c.hits++; }
};

struct Tag {
  int code;
  template <class U> Tag(const U &u) : code(u.degrees + 100) {}
};

struct Name {
  int len;
  Name(const char *s) : len(0) {
    while (s[len]) {
      ++len;
    }
  }
};

int main() {
  std::pair<long, unsigned> widened(-1, 2u);
  assert(widened.first == -1L);
  assert(widened.second == 2u);

  std::pair<Celsius, Celsius> from_ints(1, 2);
  assert(from_ints.first.degrees == 1);
  assert(from_ints.second.degrees == 2);

  Meters m{7};
  std::pair<int, long> from_operators(m, m);
  assert(from_operators.first == 7);
  assert(from_operators.second == 7L);

  std::pair<long, double> from_moved_pair = std::make_pair(3, 4);
  assert(from_moved_pair.first == 3L);
  assert(from_moved_pair.second == 4.0);

  std::pair<int, int> ints(5, 6);
  std::pair<Celsius, long> from_copied_pair = ints;
  assert(from_copied_pair.first.degrees == 5);
  assert(from_copied_pair.second == 6L);

  std::pair<Meters, Meters> meters(Meters{8}, Meters{9});
  std::pair<int, int> from_operator_pair = meters;
  assert(from_operator_pair.first == 8);
  assert(from_operator_pair.second == 9);

  std::pair<Celsius, Celsius> celsius(10, 11);
  std::pair<Label, Owner> from_record_pair = celsius;
  assert(from_record_pair.first.id == 10);
  assert(from_record_pair.second.copied == 11);
  assert(from_record_pair.second.taken == 0);

  std::pair<Label, Owner> from_moved_record_pair =
      std::make_pair(Celsius(12), Celsius(13));
  assert(from_moved_record_pair.first.id == 12);
  assert(from_moved_record_pair.second.taken == 13);
  assert(from_moved_record_pair.second.copied == 0);

  std::pair<Holder, Holder> from_by_value_pair = celsius;
  assert(from_by_value_pair.first.value == 10);
  assert(from_by_value_pair.second.value == 11);

  std::pair<Holder, Holder> from_moved_by_value_pair =
      std::make_pair(Celsius(14), Celsius(15));
  assert(from_moved_by_value_pair.first.value == 14);
  assert(from_moved_by_value_pair.second.value == 15);

  Fallback defaulted;
  assert(defaulted.value == -1);
  std::pair<Fallback, long> from_defaulted_ctor = ints;
  assert(from_defaulted_ctor.first.value == 5);
  assert(from_defaulted_ctor.second == 6L);

  Counter counter{3};
  std::pair<Observer, int> forwarded_mutable_ref(counter, 0);
  assert(forwarded_mutable_ref.first.seen == 3);
  assert(counter.hits == 4);

  std::pair<Tag, Tag> from_template_ctor = celsius;
  assert(from_template_ctor.first.code == 110);
  assert(from_template_ctor.second.code == 111);

  Celsius lvalue_source(16);
  Celsius moved_source(17);
  std::pair<Label, Owner> forwarded_records(lvalue_source,
                                            std::move(moved_source));
  assert(forwarded_records.first.id == 16);
  assert(forwarded_records.second.taken == 17);
  assert(forwarded_records.second.copied == 0);

  std::pair<Name, Name> from_string_literals("ab", "cde");
  assert(from_string_literals.first.len == 2);
  assert(from_string_literals.second.len == 3);
  return 0;
}
