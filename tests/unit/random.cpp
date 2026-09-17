struct Pair {
  int x;
  int y;
  int a[5];
  int &r;
  int *p;
  struct Pair *pair;
  int *ap[2];

  void method() {
    this->x++;
    ++y;
    this->a[4] = 1;
    this->r = 1;
    this->p = nullptr;
    this->pair = nullptr;
    this->ap[0] = nullptr;
  }

  int as_val() { return x; }

  int &as_ref() { return x; }

  int *as_ptr() { return &x; }
};

int zero() { return 0; }

struct X1 {};

void foo(int x1, int &x2, int *x3, Pair &p2, Pair *p3) {}

int main() {
  int x1 = 1;
  int c1 = x1;
  int &rx1 = x1;
  int *px1 = &x1;
  int x2 = rx1;
  int &rx2 = rx1;
  int *px2 = &rx1;
  [[maybe_unused]] int x3 = *px1;
  int &rx3 = *px1;
  int *px3 = px1;
  [[maybe_unused]] int res = x1 + x2;
  res = x1 + x2;
  Pair y1 = {1, 2, {1, 2, 3, 4, 5}, x1, nullptr, nullptr, {nullptr, nullptr}};
  [[maybe_unused]] Pair y4 = {y1.x,
                              y1.y,
                              {y1.a[0], y1.a[1], y1.a[2], y1.a[3], y1.a[4]},
                              y1.r,
                              y1.p,
                              y1.pair,
                              {y1.ap[0], y1.ap[1]}};
  Pair &ry1 = y1;
  Pair *py1 = &y1;
  Pair y2 = {ry1.x,
             ry1.y,
             {ry1.a[0], ry1.a[1], ry1.a[2], ry1.a[3], ry1.a[4]},
             ry1.r,
             ry1.p,
             ry1.pair,
             {ry1.ap[0], ry1.ap[1]}};
  [[maybe_unused]] Pair &ry2 = ry1;
  [[maybe_unused]] Pair *py2 = &ry1;
  Pair y3 = {py1->x,
             py1->y,
             {py1->a[0], py1->a[1], py1->a[2], py1->a[3], py1->a[4]},
             py1->r,
             py1->p,
             py1->pair,
             {py1->ap[0], py1->ap[1]}};
  [[maybe_unused]] Pair &ry3 = *py1;
  Pair *py3 = py1;
  py3 = nullptr;
  [[maybe_unused]] Pair *ptr2pair = py3;
  foo(x1, x1, &x1, y1, &y1);
  foo(rx1, rx1, &rx1, ry1, &ry1);
  foo(*px1, *px1, px1, *py1, py1);
  int &cr1 = c1;
  int *cp1 = &c1;
  x1 = c1;
  x1 = 1;
  x1 = cr1;
  x1 = *cp1;
  rx1 = c1;
  rx2 = cr1;
  rx3 = *cp1;
  *px1 = c1;
  *px2 = cr1;
  *px3 = *cp1;
  px1 = &c1;
  px2 = &cr1;
  px3 = cp1;
  y1.x = 2;
  y1.y = 3;
  y1.a[0] = 100;
  y1.r = 10;
  y1.p = px3;
  px3 = px2;
  y1.pair = &y3;
  (*y1.pair).x = 100;
  (*y1.pair).pair = &y2;
  (*(*y1.pair).pair).x = 100;
  y1.ap[0] = &x1;
  y1.ap[1] = &x2;
  *y1.ap[0] = 0;
  c1 = x1 + 1;
  int j = 0;
  Pair new_y = {1, 2, {1, 2, 3, 4, 5}, j, nullptr, nullptr, {nullptr, nullptr}};
  y1.x = new_y.x;
  unsigned i = 1;
  y1.a[i] = -1;
  x1++;
  ++x1;
  y1.x++;
  y1.pair->pair = &y2;
  y1.pair->pair->x = 10;
  y1.method();
  y1.pair = &y2;
  y2.pair = &y3;
  y1.pair->pair->method();
  [[maybe_unused]] X1 x{};
  [[maybe_unused]] X1 y{};
  x1 = zero() + y1.x;
  y1.x = zero() + 5;
  [[maybe_unused]] int **ptr2ptr_1 = &px1;
  [[maybe_unused]] Pair **ptr2ptr_2 = &py1;
  return 0;
}
