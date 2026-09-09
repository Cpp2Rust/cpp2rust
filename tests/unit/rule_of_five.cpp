#include <cassert>
#include <utility>

static int alive = 0;
static int copies = 0;
static int moves = 0;

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
  Buffer(Buffer &&o) : size(o.size) {
    for (int i = 0; i < 4; ++i) {
      data[i] = o.data[i];
      o.data[i] = -1;
    }
    o.size = 0;
    ++alive;
    ++moves;
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
  Buffer &operator=(Buffer &&o) {
    if (this == &o) {
      return *this;
    }
    size = o.size;
    for (int i = 0; i < 4; ++i) {
      data[i] = o.data[i];
      o.data[i] = -1;
    }
    o.size = 0;
    ++moves;
    return *this;
  }
};

static Buffer make(int size) {
  Buffer b(size);
  return std::move(b);
}

int main() {
  {
    Buffer a(4);
    Buffer b = a;
    assert(alive == 2 && copies == 1 && moves == 0);
    b.data[0] = 100;
    assert(a.data[0] == 0);

    Buffer c = std::move(a);
    assert(alive == 3 && moves == 1);
    assert(a.size == 0 && a.data[0] == -1);
    assert(c.size == 4 && c.data[3] == 3);

    Buffer d = make(2);
    assert(d.size == 2 && moves == 2);

    d = b;
    assert(d.size == 4 && d.data[0] == 100 && copies == 2);
    d = std::move(c);
    assert(d.data[0] == 0 && c.size == 0 && moves == 3);
    d = std::move(d);
    assert(d.size == 4 && moves == 3);
  }
  assert(alive == 0);
  return 0;
}
