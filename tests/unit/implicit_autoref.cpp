#include <cassert>
#include <vector>

struct Holder {
  std::vector<int> v;
};

static void write_through(int *p) { *p = 42; }

int main() {
  std::vector<int> v;
  v.push_back(10);
  v.push_back(20);

  std::vector<int> *p = &v;
  int a = (*p)[0];
  (*p)[1] = 30;

  Holder h;
  h.v.push_back(40);
  h.v.push_back(50);
  Holder *hp = &h;
  int b = (*hp).v[0];
  (*hp).v[1] = 60;

  assert(a == 10);
  assert((*p)[1] == 30);
  assert(b == 40);
  assert((*hp).v[1] == 60);

  write_through(&p->at(0));
  assert((*p)[0] == 42);
  return 0;
}
