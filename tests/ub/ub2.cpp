// panic-ub: refcount
// nondet-result: unsafe
int *null() {
  int *p = nullptr;
  return p;
}
int main() {
  int *x = null();
  return *x;
}
