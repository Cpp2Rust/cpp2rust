struct Pointers {
  int *x1;
  const int *x2;
  int *x3[5];
  const int *x4[10];
  int x5;
};

int main() {
  Pointers *default_pointers = new Pointers[10];
  delete[] default_pointers;
  return 0;
}
