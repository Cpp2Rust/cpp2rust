// panic: refcount
#include <cassert>
#include <map>
#include <string>
#include <vector>

struct S {
  int tag;
  std::vector<int> v;
  std::string s;
  std::map<int, int> m;
};

static void add(std::vector<int> *v, int k) { v->push_back(k); }

static void append(std::string *s, const char *t, std::size_t n) {
  s->append(t, n);
}

static void put(std::map<int, int> *m, int k, int v) { (*m)[k] = v; }

static void run(S *h) {
  add(&h->v, h->tag);
  append(&h->s, "ab", 2);
  put(&h->m, h->tag, 2);
  std::vector<int> *pv = &h->v;
  pv->push_back((int)pv->size());
  assert(h->v.size() == 2 && h->v[0] == 7 && h->v[1] == 1);
  assert(h->s == "ab");
  assert(h->m[7] == 2);
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
