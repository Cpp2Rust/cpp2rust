// Test that iterating over one struct field while mutating another
// does not cause a refcell double borrow error.
#include <vector>

struct S {
  std::vector<int> v;
  int a;
};

int main() {
  S s;
  s.v.push_back(1);
  for (auto e : s.v) {
    s.a++;
  }
  return 0;
}
