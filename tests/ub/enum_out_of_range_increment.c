// panic-ub

enum color { RED, GREEN, BLUE };

int main() {
  enum color c = BLUE;
  c++;
  return c == RED ? 0 : 1;
}
