// no-compile: refcount
#include <assert.h>

struct node_a {
  int n;
};

struct node_b {
  void *data;
  struct node_b *next;
};

int main(void) {
  struct node_a a = {123};

  union {
    struct node_a *to_a;
    struct node_b *to_b;
  } ptr;

  ptr.to_a = &a;
  struct node_b *out = ptr.to_b;

  assert((void *)out == (void *)&a);
  return 0;
}
