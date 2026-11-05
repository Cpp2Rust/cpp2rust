#include <cassert>
#include <vector>

struct Foo {
  std::vector<int> a;
  std::vector<int> b;
};

static void exchange(std::vector<int> &x, std::vector<int> &y) { x.swap(y); }

int main() {
  std::vector<int> v = {1, 2, 3};
  std::vector<int> w = {4};
  v.swap(w);
  assert(v.size() == 1 && v[0] == 4 && w.size() == 3 && w[2] == 3);
  Foo foo;
  foo.a.push_back(5);
  foo.b.push_back(6);
  foo.b.push_back(7);
  foo.a.swap(foo.b);
  assert(foo.a.size() == 2 && foo.a[1] == 7 && foo.b[0] == 5);
  exchange(foo.a, v);
  assert(foo.a.size() == 1 && foo.a[0] == 4 && v[1] == 7);
  return 0;
}
