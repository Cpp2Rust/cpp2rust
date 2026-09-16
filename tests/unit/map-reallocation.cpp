#include <assert.h>
#include <map>

// In the unsafe translation, Vec reallocation after enough insertions
// invalidates all raw pointers. Dereferencing a dangling pointer is undefined
// behavior: it may segfault if the page was unmapped, or read stale/wrong
// data if the allocator reused the memory.
//
// In the refcount translation, insertions shift elements so the offset points
// to the wrong element, resulting in wrong values.

int main() {
  const int N = 10000;
  const int sentinel = N / 2;
  std::map<int, int> m;

  // Insert a sentinel element
  m[sentinel] = sentinel;

  // Get an iterator and a raw pointer to the value
  std::map<int, int>::iterator it = m.find(sentinel);
  int *p = &it->second;

  // Verify before insertions
  assert(it->second == sentinel &&
         "iterator does not have correct value before insert");
  assert(*p == sentinel && "pointer does not have correct value before insert");

  // Insert elements to invalidate the above iterator in Rust.
  for (int i = 0; i < sentinel; ++i) {
    m[i] = i;
  }
  for (int i = sentinel + 1; i <= N; ++i) {
    m[i] = i;
  }

  // After N insertions:
  // - C++: iterator and pointer are still valid (tree nodes don't move)
  // - Unsafe: Vec reallocated multiple times, raw pointer p is dangling => UB
  // - Refcount: sentinel was at index 0, now at index N/2 due to shifts,
  //   but Ptr still stores offset 0 => reads key 0 instead of key N/2
  assert(it->second != 0 &&
         "in refcount, iterator points to index 0 instead of sentinel");
  assert(it->second == sentinel &&
         "iterator does not have correct value after insert");
  assert(*p == sentinel && "pointer does not have correct value after insert");

  // Also verify we can mutate through the iterator after all those insertions
  it->second = 0xDEAD;
  assert(m[sentinel] == 0xDEAD);
  assert(*p == 0xDEAD);

  // Verify the map is correctly sorted and has all elements
  assert(m.size() == (unsigned)(N + 1));
  int prev = -1;
  for (auto &pair : m) {
    assert(pair.first > prev);
    prev = pair.first;
  }

  return 0;
}
