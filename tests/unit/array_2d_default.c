#include <assert.h>

static void fill_row(char *row, char c) {
  row[0] = c;
  row[1] = '\0';
}

int main(void) {
  char grid[3][6];
  for (int i = 0; i < 3; i++) {
    fill_row(grid[i], (char)('a' + i));
  }
  assert(grid[0][0] == 'a');
  assert(grid[1][0] == 'b');
  assert(grid[2][0] == 'c');
  assert(grid[1][1] == '\0');
  grid[2][5] = 'z';
  assert(grid[2][5] == 'z');
  return 0;
}
