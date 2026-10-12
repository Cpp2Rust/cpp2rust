struct A {};

struct D {
  void operator()(const A *ptr) const { delete ptr; }
};

int main() {
  int *d = new int(0);
  *d = 5;
  delete d;
  const int *c = new int(3);
  delete c;
  D{}(new A);
  return 0;
}
