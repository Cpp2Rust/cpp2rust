#include <cassert>
#include <vector>

struct Counter {
  int n;
  int get() const { return n; }
  void add(int k) { n += k; }
  Counter *self() { return this; }
  void take(Counter *other) {
    n += other->n;
    other->n = 0;
  }
};

struct S {
  int tag;
  Counter c;
  Counter arr[2];
  std::vector<int> v;
  void bump() { c.add(tag); }
};

static void run(S *o) {
  o->c.add(2);
  assert(o->c.get() == 2);
  o->arr[1].add(5);
  assert(o->arr[1].get() == 5);
  o->c.self()->add(1);
  assert(o->c.get() == 3);
  assert(o->c.self() == &o->c);
  o->arr[0].take(&o->arr[1]);
  assert(o->arr[0].get() == 5 && o->arr[1].get() == 0);
  o->c.take(&o->c);
  assert(o->c.get() == 0);
  o->bump();
  assert(o->c.get() == 1);
  o->v.push_back(o->c.get());
  assert(o->v.size() == 1 && o->v[0] == 1);
  assert(o->tag == 1);
}

int main() {
  S local{};
  local.tag = 1;
  run(&local);
  S *heap = new S{};
  heap->tag = 1;
  run(heap);
  delete heap;
  return 0;
}
