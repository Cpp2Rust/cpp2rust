// panic-ub: refcount
// nondet-result: unsafe
int main() {
  int arr[3] = {1, 2, 3};
  int *p = arr;
#pragma GCC diagnostic ignored "-Wfree-nonheap-object"
  delete[] p;
  return 0;
}
