#include <map>

int default_constructions = 0;

struct Key {
  int order;
  int identity;
};

bool operator<(const Key &a, const Key &b) { return a.order < b.order; }
bool operator==(const Key &a, const Key &b) { return a.identity == b.identity; }

struct Mapped {
  int value;
  Mapped() : value(0) { ++default_constructions; }
};

int main() {
  bool ok = true;

  const Key distinct_a{1, 7};
  const Key distinct_b{2, 7};
  ok &= (distinct_a == distinct_b);

  std::map<Key, Mapped> distinct;
  Mapped &a = distinct[distinct_a];
  a.value = 11;
  auto a_it = distinct.find(distinct_a);
  Mapped *a_value = &a_it->second;
  ok &= (default_constructions == 1);

  Mapped &b = distinct[distinct_b];
  b.value = 22;
  auto b_it = distinct.find(distinct_b);
  ok &= (default_constructions == 2);
  ok &= (distinct.size() == 2);
  ok &= (&a != &b);
  ok &= (a.value == 11 && b.value == 22);
  ok &= (a_it == distinct.find(distinct_a));
  ok &= (b_it == distinct.find(distinct_b));
  ok &= (b_it != distinct.end());
  ok &= (&a_it->second == a_value);
  ok &= (a_it->second.value == 11);
  ok &= (distinct_a == distinct_b);

  std::size_t distinct_count = 0;
  for (const auto &entry : distinct) {
    if (distinct_count == 0)
      ok &= (entry.first.order == 1 && entry.second.value == 11);
    else if (distinct_count == 1)
      ok &= (entry.first.order == 2 && entry.second.value == 22);
    ++distinct_count;
  }
  ok &= (distinct_count == 2);

  const Key equivalent_a{3, 8};
  const Key equivalent_b{3, 9};
  ok &= !(equivalent_a == equivalent_b);

  std::map<Key, Mapped> equivalent;
  const int before_equivalent = default_constructions;
  Mapped &first = equivalent[equivalent_a];
  first.value = 31;
  auto first_it = equivalent.find(equivalent_a);
  Mapped *first_value = &first_it->second;
  ok &= (default_constructions == before_equivalent + 1);

  Mapped &second = equivalent[equivalent_b];
  const bool reused = default_constructions == before_equivalent + 1;
  ok &= reused;
  ok &= (equivalent.size() == 1);
  if (reused) {
    ok &= (first_it != equivalent.end());
    ok &= (&first == &second);
    ok &= (first_it == equivalent.find(equivalent_a));
    ok &= (first_it == equivalent.find(equivalent_b));
    ok &= (&first_it->second == first_value);
    ok &= (first_it->second.value == 31 && second.value == 31);

    std::size_t equivalent_count = 0;
    for (const auto &entry : equivalent) {
      ok &= (entry.first.order == 3 && entry.second.value == 31);
      ++equivalent_count;
    }
    ok &= (equivalent_count == 1);
  }
  ok &= !(equivalent_a == equivalent_b);

  const Key later_key{4, 10};
  Mapped &later = equivalent[later_key];
  later.value = 41;
  ok &= (default_constructions == before_equivalent + 2);
  ok &= (equivalent.size() == 2);
  if (reused) {
    ok &= (first_it == equivalent.find(equivalent_a));
    ok &= (&first_it->second == first_value);
    ok &= (first_it->second.value == 31);
  }

  const Key matching_a{5, 5};
  const Key matching_b{5, 5};
  const Key matching_c{6, 6};
  const int before_matching = default_constructions;
  std::map<Key, Mapped> matching;
  Mapped &matched = matching[matching_a];
  matched.value = 51;
  auto matched_it = matching.find(matching_a);
  Mapped *matched_value = &matched_it->second;
  ok &= (default_constructions == before_matching + 1);

  Mapped &matched_again = matching[matching_b];
  ok &= (default_constructions == before_matching + 1);
  ok &= (matching.size() == 1);
  ok &= (&matched == &matched_again);
  ok &= (matched_it == matching.find(matching_b));
  ok &= (&matched_it->second == matched_value);
  ok &= (matched_it->second.value == 51);

  Mapped &other = matching[matching_c];
  other.value = 61;
  ok &= (default_constructions == before_matching + 2);
  ok &= (matching.size() == 2);
  ok &= (matched_it == matching.find(matching_a));
  ok &= (&matched_it->second == matched_value);
  ok &= (matched_it->second.value == 51);

  return ok ? 0 : 1;
}
