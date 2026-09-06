#include <cassert>

int global = 0;

struct S {
  ~S() { global++; }
};

struct Defaulted {
  S s;
  ~Defaulted() = default;
};

struct Middle {
  S s;
};

struct Outer {
  Middle m;
};

struct ArrayMember {
  S items[3];
};

struct EmptyBody {
  S s;
  ~EmptyBody() {}
};

template <typename T> struct Templated {
  T v;
  ~Templated() { global += sizeof(T); }
};

struct Copied {
  int v;
  ~Copied() { global++; }
};

int order[3];
int order_count = 0;

struct Tagged {
  int tag;
  ~Tagged() { order[order_count++] = tag; }
};

struct Ordered {
  Tagged first;
  int dummy1;
  Tagged second;
  int dummy2;
  Tagged third;
};

int main() {
  {
    S s{};
  }
  assert(global == 1);

  {
    S s{};
  }
  assert(global == 2);

  {
    Defaulted d{};
  }
  assert(global == 3);

  {
    Outer o{};
  }
  assert(global == 4);

  {
    ArrayMember am{};
  }
  assert(global == 7);

  {
    EmptyBody e{};
  }
  assert(global == 8);

  {
    Templated<char> tc{};
    Templated<int> ti{};
  }
  assert(global == 13);

  {
    Copied a{5};
    Copied b = a;
    assert(b.v == 5);
  }
  assert(global == 15);

  {
    Ordered o{{1}, 0, {2}, 0, {3}};
  }
  assert(order_count == 3);
  assert(order[0] == 3);
  assert(order[1] == 2);
  assert(order[2] == 1);

  return 0;
}
