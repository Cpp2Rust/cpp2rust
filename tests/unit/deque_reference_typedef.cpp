#include <cassert>
#include <deque>

int main() {
  std::deque<int> d;
  int first = 1;
  int last = 2;
  d.push_back(first);
  d.push_back(last);
  std::deque<int>::reference r = d.front();
  r = 5;
  std::deque<int>::const_reference c = d.back();
  assert(d.front() == 5 && c == 2);
  return 0;
}
