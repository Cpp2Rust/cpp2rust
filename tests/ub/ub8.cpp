// panic-ub: refcount
// nondet-result: unsafe
int main() {
  int x = 5;
  int *p = &x;
  return *++p;
}
