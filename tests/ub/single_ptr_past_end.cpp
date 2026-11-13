// panic-ub: refcount
// nondet-result: unsafe
int main() {
  int x = 7;
  int y = 8;
  int *p = &x;
  p[1] = 3;
  return x + y;
}
