#include <cassert>
#include <vector>

int main() {
    std::vector<std::vector<int>> v1;
    std::vector<int> v2 = {1};
    v1.push_back(v2);

    assert(v1.back().at(0) == 1);
    return 0;
}
