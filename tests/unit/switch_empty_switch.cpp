#include <cassert>

int empty_switch(int x) {
  switch (x) {}
  return x;
}

int main() {
  assert(empty_switch(5) == 5);
  return 0;
}
