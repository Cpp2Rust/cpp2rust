#include <cassert>

void unused_param(int x) { (void)x; }

int side_effect_counter = 0;
int bump_and_return() {
  ++side_effect_counter;
  return side_effect_counter;
}

struct Holder {
  int field;
};

int main() {
  unused_param(42);

  int y = 5;
  (void)y;

  int z = ((void)y, 7);
  assert(z == 7);

  int counter = 0;
  int w = ((void)counter, counter = 3, counter);
  assert(w == 3);
  assert(counter == 3);

  (void)bump_and_return();
  assert(side_effect_counter == 1);

  int v = ((void)bump_and_return(), 99);
  assert(side_effect_counter == 2);
  assert(v == 99);

  (void)0;
  (void)(0);

  (void)(y);

  ((void)0);
  ((void)(y));

  int err = 0;
  ((void)(err = 42));
  assert(err == 42);

  int chosen = ((void)(err = 7), 123);
  assert(err == 7);
  assert(chosen == 123);

  (void)bump_and_return;
  assert(side_effect_counter == 2);

  (void)(&bump_and_return);
  assert(side_effect_counter == 2);

  (void)(static_cast<int (*)()>(&bump_and_return));
  assert(side_effect_counter == 2);

  int storage = 11;
  int *p = &storage;
  (void)(*p);
  (void)(p);

  int arr[] = {1, 2, 3};
  (void)(arr[1]);

  Holder h{17};
  (void)(h.field);
  Holder *hp = &h;
  (void)(hp->field);

  return 0;
}
