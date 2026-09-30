#include <cassert>
#include <map>
#include <string>
#include <vector>

struct S {
  int tag;
  std::vector<int> v;
};

static void add(std::vector<int> *v, int k) { v->push_back(k); }

static void run(S *h) {
  add(&h->v, h->tag);
  std::vector<int> *pv = &h->v;
  pv->push_back((int)pv->size());
  assert(h->v.size() == 2 && h->v[0] == 7 && h->v[1] == 1);
  assert(h->tag == 7);
}

int main() {
  S local;
  local.tag = 7;
  run(&local);
  S *heap = new S();
  heap->tag = 7;
  run(heap);
  delete heap;
  return 0;
}
