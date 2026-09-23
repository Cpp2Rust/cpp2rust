// nondet-result: unsafe, refcount
int main() {
  int *arr = new int[10];
  int *ptr = arr + 1;
  int out = *ptr;
#pragma GCC diagnostic ignored "-Wfree-nonheap-object"
  delete[] ptr;
  return out;
}
