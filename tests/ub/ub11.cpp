// panic-ub: refcount
// nondet-result: unsafe
int main() {
  int *element = new int(10);
  int *ptr = element + 1;
  int out = *ptr;
  delete element;
  return out;
}
