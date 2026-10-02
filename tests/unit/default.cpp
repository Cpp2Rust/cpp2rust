struct Pointers {
  int *x1;
  const int *x2;
  int *x3[5];
  const int *x4[10];
  int x5;
};

struct SmallArrays {
  int a[32];
  int (*f)(int);
  int (*fs[2])(int);
  Pointers p[2];
};

struct BigArray {
  int a[33];
};

int main() {
  Pointers *default_pointers = new Pointers[10];
  delete[] default_pointers;
  SmallArrays *small = new SmallArrays[2];
  delete[] small;
  BigArray *big = new BigArray[2];
  delete[] big;
  return 0;
}
