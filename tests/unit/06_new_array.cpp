int main() {
  int *e = new int[2];
  e[0] = 6;
  e[1] = 7;
  delete[] e;
  return 0;
}
