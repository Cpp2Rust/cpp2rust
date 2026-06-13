#include <cassert>
#include <vector>

struct NonCopy {
  std::vector<int> data;
  int tag = 0;
};

int main() {
  NonCopy arr[3];
  arr[0].tag = 7;
  arr[1].data.push_back(42);
  assert(arr[0].tag == 7);
  assert(arr[1].data.size() == 1);
  assert(arr[1].data[0] == 42);
  assert(arr[2].tag == 0);
  assert(arr[2].data.size() == 0);
  return 0;
}
