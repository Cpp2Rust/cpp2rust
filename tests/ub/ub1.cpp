// panic-ub: refcount
// nondet-result: unsafe
int &dangling() {
  int x = 1;
  int *p = &x;
  return *p;
}
int main() {
  int &x = dangling();
  return x;
}
