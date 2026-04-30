#include <cassert>
#include <vector>

struct Outer {
  typedef struct {
    int block_idx;
    int num_extra_zero_runs;
  } RunInfo;

  std::vector<RunInfo> runs;
};

int main() {
  Outer o;

  Outer::RunInfo info;
  info.block_idx = 1;
  info.num_extra_zero_runs = 2;
  o.runs.push_back(info);

  assert(o.runs.size() == 1);
  assert(o.runs[0].block_idx == 1);
  assert(o.runs[0].num_extra_zero_runs == 2);

  return 0;
}
