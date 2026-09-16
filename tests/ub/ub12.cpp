// panic-ub: refcount
// nondet-result: unsafe
void escape(int *ptr) { delete ptr; }

int main() {
  int *alloc = new int(1);
  escape(alloc);
  delete alloc;
  return 0;
}
