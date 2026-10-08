#include <cassert>
#include <cstddef>

struct list_head {
  list_head *next;
};

struct node {
  int value;
  int *value_ptr;
};

struct node_with_defaults {
  node_with_defaults *next = nullptr;
  int value = 3;
};

int main() {
  list_head list = {&list};
  assert(list.next == &list);

  node n = {42, &n.value};
  assert(n.value_ptr == &n.value);
  *n.value_ptr = 7;
  assert(n.value == 7);

  list_head arr[2] = {{&arr[1]}, {&arr[0]}};
  assert(arr[0].next == &arr[1]);
  assert(arr[1].next == &arr[0]);

  node_with_defaults d{&d};
  assert(d.next == &d);
  assert(d.value == 3);

  size_t size = sizeof(size);
  assert(size == sizeof(size_t));

  return 0;
}
