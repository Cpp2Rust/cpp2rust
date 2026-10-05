#include <cassert>
#include <vector>

struct node {
  int value;
  node *next;
};

struct pair_t {
  int a;
  node *n;
};

node *global_node;

node *id(node *n) { return n; }

node *pick(node *a, node *b) { return a ? a : b; }

node *local_ptr(node *n) {
  node *p = n->next;
  return p;
}

node *call_once(node *n, node *m) { return pick(n, m); }

node *call_twice(node *n) { return pick(n, n); }

node *next_of(node *n) { return n->next; }

node *ret_global() { return global_node; }

node *address_taken(node *n) {
  node **pp = &n;
  return *pp ? n : nullptr;
}

pair_t ret_struct(int a, node *n) {
  pair_t p;
  p.a = a;
  p.n = n;
  return p;
}

pair_t ret_struct_param(pair_t p) { return p; }

pair_t ret_struct_ref(pair_t &p) { return p; }

std::vector<int> ret_vec(std::vector<int> v) { return v; }

std::vector<int> ret_vec_local() {
  std::vector<int> v;
  v.push_back(1);
  return v;
}

node *ret_loop(node *n) {
  while (n->next) {
    if (n->value == 2)
      return n;
    n = n->next;
  }
  return n;
}

int main() {
  node c{3, nullptr};
  node b{2, &c};
  node a{1, &b};
  global_node = &a;
  assert(id(&a) == &a);
  assert(local_ptr(&a) == &b);
  assert(call_once(nullptr, &b) == &b);
  assert(call_twice(&c) == &c);
  assert(next_of(&b) == &c);
  assert(ret_global() == &a);
  assert(address_taken(&a) == &a);
  pair_t p = ret_struct(4, &a);
  assert(p.a == 4 && p.n == &a);
  pair_t q = ret_struct_param(p);
  assert(q.a == 4 && q.n == &a);
  pair_t r = ret_struct_ref(q);
  assert(r.a == 4 && r.n == &a);
  assert(ret_vec(ret_vec_local()).size() == 1);
  assert(ret_loop(&a) == &b);
  return 0;
}
