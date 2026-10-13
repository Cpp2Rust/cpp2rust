#include <cassert>
#include <deque>
#include <string>
#include <vector>

int main() {
  std::vector<std::deque<std::vector<std::string>>> v;
  v.emplace_back();
  v.emplace_back();

  std::vector<std::string> row;
  row.push_back("a");
  row.push_back("bc");
  v[0].push_back(row);
  row.push_back("def");
  v[1].push_back(row);
  v[1].emplace_back();
  v[1].back().push_back(v[0].front()[1] + "x");

  assert(v[0].front().size() == 2);
  assert(v[1].front().size() == 3 && v[1].front()[2] == "def");
  assert(v[1].back()[0] == "bcx");

  auto copy = v;
  copy[0].front()[0] = std::string("z");
  v[0].front()[1].at(0) = 'B';
  assert(v[0].front()[0] == "a" && copy[0].front()[0] == "z");
  assert(v[0].front()[1] == "Bc" && copy[0].front()[1] == "bc");

  v[1].pop_front();
  assert(v[1].front()[0] == "bcx");
  assert(copy[1].front().size() == 3);
  assert(row.size() == 3);
  return 0;
}
