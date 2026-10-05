#include <cassert>

struct Inner {
  int a;
  int b;
};

struct Cookie {
  int x;
  int *data;
  const int *in;
  Inner inner;
  int arr[4];
};

struct Counter {
  int n = 0;
  void inc() { ++n; }
};

static void set(int *p, int v) { *p = v; }

static int get(const Cookie &c) { return c.x; }

static int first(const int *p) { return p[0]; }

// Unboxed: only the fields of the copy are accessed.
static int consume(const Cookie &cookie) {
  Cookie c = cookie;
  int sum = 0;
  for (int i = 0; i < 4; ++i) {
    c.data[i] = c.in[i] * c.x;
    sum += c.data[i];
  }
  c.x = sum;
  c.inner.a += c.x;
  c.arr[1] = c.inner.a;
  *c.data = c.arr[1] + first(c.in);
  set(c.data + 1, first(c.data));
  return c.x + c.inner.b;
}

int main() {
  int data[4] = {0, 0, 0, 0};
  int in[4] = {1, 2, 3, 4};

  // Boxed: passed by reference.
  Cookie cookie;
  cookie.x = 2;
  cookie.data = data;
  cookie.in = in;
  cookie.inner.a = 1;
  cookie.inner.b = 3;
  assert(consume(cookie) == 23);
  assert(data[0] == 22 && data[1] == 22 && data[3] == 8);
  assert(get(cookie) == 2);

  // Unboxed: the fields are only read and written.
  Inner local;
  local.a = 4;
  local.b = local.a * 2;
  local.a++;
  assert(local.a + local.b == 13);

  // Boxed: a pointer to one of its fields is made.
  Inner field_addr = {0, 0};
  set(&field_addr.b, 7);
  assert(field_addr.b == 7);

  // Boxed: a method is called on it.
  Counter counter;
  counter.inc();
  assert(counter.n == 1);
  return 0;
}
