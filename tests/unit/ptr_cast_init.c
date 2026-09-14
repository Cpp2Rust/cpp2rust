#include <assert.h>
#include <stddef.h>

struct header {
  int tag;
  int size;
};

struct view {
  int tag;
};

struct entry {
  int id;
};

static const struct entry e0 = {1};
static const struct entry e1 = {2};
static const struct entry *registry[] = {&e0, &e1, NULL};

static void get_registry(const struct entry ***out) {
  *out = (const struct entry **)&registry;
}

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

  const struct entry **avail = NULL;
  get_registry(&avail);
  assert(avail != NULL);
  assert(avail[0]->id == 1);
  assert(avail[1]->id == 2);
  assert(avail[2] == NULL);

  return 0;
}
