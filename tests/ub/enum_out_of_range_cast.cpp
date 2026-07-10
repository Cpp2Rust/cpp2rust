// panic-ub

enum Color { RED, GREEN, BLUE };

int main() {
  int n = 3;
  Color c = (Color)n;
  return c == BLUE ? 0 : 1;
}
