#include <cassert>
#include <cstring>
#include <vector>

struct Inner {
  int a;
  char name[8];
};

struct Header {
  int tag;
  short size;
};

struct Outer {
  int x;
  Inner inner;
  Inner items[3];
  std::vector<int> v;
  int *cursor;
  int buf[4];

  int next() { return buf[x++]; }
  int sum() const { return x + inner.a; }
  void push(int k) { v.push_back(k + x); }
};

static void set(int *p, int value) { *p = value; }

static int bump(Outer *o) {
  o->x++;
  return 1;
}

int main() {
  Outer o{};
  o.x = 1;

  // Pointers to fields, nested fields, and fields of array elements.
  int *px = &o.x;
  *px = 2;
  assert(o.x == 2);
  int *pa = &o.inner.a;
  set(pa, 3);
  assert(o.inner.a == 3);
  int *pi = &o.items[1].a;
  *pi = 4;
  assert(o.items[1].a == 4);
  assert(&o.items[0].a != pi);

  // Arrays in fields decay to pointers to their elements.
  char *name = o.inner.name;
  for (int i = 0; i < 3; ++i) {
    name[i] = 'a' + i;
  }
  assert(strlen(o.inner.name) == 3);
  assert(name + 3 == &o.inner.name[3]);

  // A pointer into the struct, stored in the struct itself.
  o.cursor = &o.buf[1];
  *o.cursor = 5;
  *o.cursor++ += 1;
  assert(o.buf[1] == 6 && o.cursor == &o.buf[2]);

  // Reads and writes of fields of the same struct in one statement.
  o.x = 0;
  int first = o.next();
  assert(first == 0 && o.x == 1);
  o.x = o.inner.a + bump(&o);
  assert(o.x == 4);
  o.push(o.inner.a);
  assert(o.v.size() == 1 && o.v[0] == 7);
  assert(o.sum() == 7);

  // Byte-level access to a field.
  int y = 0;
  memcpy(&o.items[2].a, &o.items[1].a, sizeof(int));
  memcpy(&y, &o.items[2].a, sizeof(int));
  assert(y == 4);

  // Writes through a reinterpreted struct pointer.
  unsigned char bytes[sizeof(Header)] = {};
  Header *view = reinterpret_cast<Header *>(bytes);
  view->tag = 0x01020304;
  assert(bytes[0] == 0x04 || bytes[3] == 0x04);
  set(&view->tag, 0);
  assert(bytes[0] == 0 && bytes[3] == 0);
  view->size = 1;
  assert(bytes[4] + bytes[5] == 1);
  return 0;
}
