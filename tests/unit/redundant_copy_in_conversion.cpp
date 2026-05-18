struct iter {
  using iterator_category = int;
  int *p;
};

struct const_iter {
  using iterator_category = int;
  const int *p;
  const_iter(const iter &o) : p(o.p) {}
  const_iter(const const_iter &) = default;
};

static void sink(const_iter i) {}

int main() {
  int buf[2] = {0, 0};
  iter it{buf};
  sink(it);
  return 0;
}
