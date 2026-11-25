#include <cstdio>

struct Guard {
  int digit;
  ~Guard() { std::printf("%d ", digit); }
  explicit operator bool() const { return digit != 0; }
};

void step(int &i) {
  std::printf("9 ");
  ++i;
}

int main() {
  std::printf("normal/continue: ");
  for (int i = 0; i < 2; step(i)) {
    Guard first{1};
    Guard second{2};
    if (i == 0)
      continue;
  }

  std::printf("\nbreak: ");
  for (int i = 0; i < 2; step(i)) {
    Guard g{3};
    break;
  }

  std::printf("\nnested for: ");
  for (int i = 0; i < 1; step(i)) {
    Guard outer{4};
    for (int j = 0; j < 2; step(j)) {
      Guard inner{5};
      if (j == 0)
        continue;
      break;
    }
    continue;
  }

  std::printf("\ninner while/do/range: ");
  for (int i = 0; i < 1; step(i)) {
    Guard outer{4};
    int j = 0;
    while (++j < 3) {
      Guard inner{5};
      if (j == 1)
        continue;
      break;
    }
    do {
      Guard inner{6};
      continue;
    } while (false);
    int values[] = {1, 2};
    for (int value : values) {
      Guard inner{value};
      if (value == 1)
        continue;
      break;
    }
    continue;
  }

  std::printf("\ncondition: ");
  for (int i = 2; Guard condition{i};
       --i, std::printf("inc%d ", condition.digit)) {
    Guard body{4};
    if (i == 2)
      continue;
  }

  std::printf("\ninit/noncompound: ");
  int i = 0;
  for (Guard init{5}; i < 2; step(i))
    Guard body{6};

  std::printf("\nno increment: ");
  for (int j = 0; j < 2;) {
    Guard g{++j};
    continue;
  }

  std::printf("\n");
  return 0;
}
