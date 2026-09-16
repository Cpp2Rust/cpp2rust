// panic-ub: refcount
// nondet-result: unsafe
int *foo(int *a) { return &a[5]; }
int main() {
  int *p1 = new int[10];
  [[maybe_unused]] int out = foo(&p1[1])[4];
  delete[] p1;
  return 0;
}
