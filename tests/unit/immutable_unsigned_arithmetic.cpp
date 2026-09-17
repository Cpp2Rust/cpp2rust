int main() {
  unsigned a = 0;
  unsigned *p = &a;
  // Unigned arithmetic is translated as: p.wrapping_add(p).
  // Both p's must be immutably borrowed because wrapping_add requires an
  // immutable refrence.
  *p += *p;
  return *p;
}
