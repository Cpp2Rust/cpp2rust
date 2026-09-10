#include <cassert>

int order[8];
int n = 0;

struct Base {
  ~Base() { order[n++] = 1; }
};

struct Member {
  ~Member() { order[n++] = 2; }
};

struct Derived : Base {
  Member m;
  ~Derived() { order[n++] = 3; }
};

struct Implicit : Base {
  Member m;
};

int main() {
  {
    Derived d;
  }
  assert(n == 3);
  assert(order[0] == 3 && order[1] == 2 && order[2] == 1);

  n = 0;
  {
    Implicit i;
  }
  assert(n == 2);
  assert(order[0] == 2 && order[1] == 1);

  n = 0;
  Derived *p = new Derived;
  delete p;
  assert(n == 3);
  assert(order[0] == 3 && order[1] == 2 && order[2] == 1);
  return 0;
}
