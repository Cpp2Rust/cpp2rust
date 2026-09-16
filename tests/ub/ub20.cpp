// panic-ub: refcount
// nondet-result: unsafe
void foo(int *single) { delete single; }
int main() {
  int *x = new int[10];
  foo(x);
  return 0;
}
