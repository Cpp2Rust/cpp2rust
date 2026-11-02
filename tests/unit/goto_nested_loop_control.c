#include <assert.h>

int main(void) {
  int visits = 0;
  int tail = 0;
  for (int i = 0; i < 3; i++) {
  outer:
    visits++;
    {
    inner:
      if (i == 0) {
        break;
      }
    }
    tail++;
  }
  assert(visits == 1);
  assert(tail == 0);

  visits = 0;
  tail = 0;
  for (int i = 0; i < 3; i++) {
    switch (i) {
    case 0: {
    switch_outer:
      visits++;
      {
      switch_inner:
        if (i == 0) {
          break;
        }
      }
      tail++;
    }
    case 1:
      tail += 10;
      break;
    default:
      tail += 100;
      break;
    }
  }
  assert(visits == 1);
  assert(tail == 110);
  return 0;
}
