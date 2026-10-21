#include <assert.h>
#include <signal.h>

static int total = 0;

void add(int x) { total += x; }

int main() {
  sig_t handler = add;
  handler(2);
  handler(3);
  assert(total == 5);
  return 0;
}
