#include <assert.h>

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
  return 0;
}
