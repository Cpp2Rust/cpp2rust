#include <assert.h>
#include <stddef.h>

struct header {
  int tag;
  int size;
};

struct view {
  int tag;
};

int main(void) {
  const char text[] = "hi";
  const char *cp = text;
  unsigned char *u = (unsigned char *)cp;
  assert(u[0] == 'h');
  assert(u[1] == 'i');

  struct header h = {7, 32};
  struct header *hp = &h;
  struct view *v = (struct view *)hp;
  assert(v->tag == 7);

  char data[] = "hi";
  void *vp = data;
  int n = 2;
  char *sel = (n < 100) ? vp : NULL;
  assert(sel != 0);
  assert(sel[0] == 'h');
  n = 200;
  sel = (n < 100) ? vp : NULL;
  assert(sel == 0);
  return 0;
}
