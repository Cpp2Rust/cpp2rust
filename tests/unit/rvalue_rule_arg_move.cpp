#include <cstdio>
#include <utility>
#include <vector>

int copies = 0;
int moves = 0;

struct Elem {
  int v;
  Elem(int v) : v(v) {}
  Elem(const Elem &o) : v(o.v) { ++copies; }
  Elem(Elem &&o) : v(o.v) { ++moves; }
  Elem &operator=(const Elem &) = default;
  Elem &operator=(Elem &&) = default;
};

int main() {
  std::vector<Elem> v;
  v.reserve(4);
  Elem a(1), b(2);
  // An argument that binds to an rvalue reference is moved, not copied.
  v.insert(v.end(), std::move(a));
  v.insert(v.end(), b);
  bool ok = copies == 1 && moves == 1 && v[0].v == 1 && v[1].v == 2;
  Elem *pa = &a;
  printf("%d %d %d\n", copies, moves, pa->v);
  return ok ? 0 : 1;
}
