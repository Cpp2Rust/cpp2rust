// panic-ub: refcount
// nondet-result: unsafe
int main() {
  int x = 1;
  int *p = &x;
#pragma GCC diagnostic ignored "-Wfree-nonheap-object"
  delete p;
  return 0;
}
