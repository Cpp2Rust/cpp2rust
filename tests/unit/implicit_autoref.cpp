#include <cstdio>

struct Counter {
  int value = 0;
  void bump() { ++value; }
  int get() const { return value; }
};

struct Holder {
  Counter c;
  Counter &ref;
  Holder(Counter &c) : ref(c) {}
};

void via_ref(Counter &r) { r.bump(); }

int main() {
  Counter c;
  Counter *p = &c;
  (*p).bump();
  p->bump();

  Counter arr[2];
  arr[0].bump();
  arr[1].bump();

  Holder h(c);
  h.c.bump();
  h.ref.bump();

  via_ref(c);

  int sum = (*p).get() + h.c.get() + h.ref.get() + arr[0].get() + arr[1].get();
  printf("%d\n", sum);
  return 0;
}
