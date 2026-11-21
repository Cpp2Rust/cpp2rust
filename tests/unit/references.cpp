#include <cassert>

struct Pair {
  int x;
  int y;
  int arr[2];
};

struct Nested {
  Pair p;
};

static int calls = 0;

static Pair make_pair() {
  ++calls;
  return {1, 2, {3, 4}};
}

static Nested make_nested() { return {make_pair()}; }

static void note(int v) { calls += v; }

// A temporary whose subobject is bound to a reference lives as long as the
// reference.
static void extended_temporaries(bool c, int v) {
  const int &field = make_pair().y;
  const int &nested = make_nested().p.x;
  const int &element = make_pair().arr[1];
  const int &chosen = c ? make_pair().x : v;
  const int &comma = (note(10), make_pair().x);
  const int &&rvalue = make_pair().y;
  static const int &once = make_pair().arr[0];
  assert(field == 2);
  assert(nested == 1);
  assert(element == 4);
  assert(chosen == 1);
  assert(comma == 1);
  assert(rvalue == 2);
  assert(once == 3);
  assert(calls == 18);
}

static const int &global_field = make_pair().x;

int main() {
  int x = 1;
  int &r = x;
  r = 5;
  assert(x == 5);
  extended_temporaries(true, 7);
  assert(global_field == 1);
  return 0;
}
