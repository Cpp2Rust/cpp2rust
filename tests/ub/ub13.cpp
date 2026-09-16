// panic-ub: refcount
// nondet-result: unsafe
void escape(int *p) { delete p; }

int main() {
  int *p1 = new int(1);
  escape(p1);
  return *p1;
}

// Pointer arithmetic errors: Incorrectly performing arithmetic operations on
// pointers, leading to invalid memory accesses and undefined behavior.
