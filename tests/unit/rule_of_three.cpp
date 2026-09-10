#include <cassert>
#include <utility>

static int alive = 0;
static int copies = 0;

struct Buffer {
  int data[4];
  int size;
  Buffer(int size) : size(size) {
    for (int i = 0; i < 4; ++i) {
      data[i] = i < size ? i : -1;
    }
    ++alive;
  }
  ~Buffer() { --alive; }
  Buffer(const Buffer &o) : size(o.size) {
    for (int i = 0; i < 4; ++i) {
      data[i] = o.data[i];
    }
    ++alive;
    ++copies;
  }
  Buffer &operator=(const Buffer &o) {
    if (this == &o) {
      return *this;
    }
    size = o.size;
    for (int i = 0; i < 4; ++i) {
      data[i] = o.data[i];
    }
    ++copies;
    return *this;
  }
};

static int sum(const Buffer &b) {
  int s = 0;
  for (int i = 0; i < b.size; ++i) {
    s += b.data[i];
  }
  return s;
}

int main() {
  {
    Buffer a(4);
    Buffer b = a;
    assert(alive == 2 && copies == 1);
    b.data[0] = 100;
    assert(a.data[0] == 0);

    Buffer c(2);
    c = a;
    assert(c.size == 4 && c.data[3] == 3);
    assert(alive == 3 && copies == 2);
    c = c;
    assert(copies == 2);

    assert(sum(a) == 6);
    assert(sum(b) == 106);

    Buffer d = std::move(a);
    assert(alive == 4 && copies == 3);
    assert(a.size == 4 && a.data[3] == 3);
    d = std::move(b);
    assert(copies == 4);
    assert(b.data[0] == 100 && d.data[0] == 100);
  }
  assert(alive == 0);
  return 0;
}
