#include <stdio.h>

int main() {
  const char *fname = "testfile.txt";
  const char *mode = "rb";
  FILE *file_ptr = fopen(fname, mode);

  return 0;
}
