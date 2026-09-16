#include <algorithm>

int main() {
  int input[] = {1, 2, 3};
  int output[3];
  std::copy(input, input + 3, output);
  return 0;
}
