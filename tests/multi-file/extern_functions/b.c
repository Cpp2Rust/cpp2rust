static int unrelated1(void) { return 1; }
static int unrelated2(void) { return 2; }
static int unrelated3(void) { return 3; }

int helper(int x) {
  (void)unrelated1();
  (void)unrelated2();
  (void)unrelated3();
  return x + 1;
}
