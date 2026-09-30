// panic: refcount
#include <assert.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>

struct In {
  short a;
  int b;
};

struct S {
  int x;
  struct In in;
  unsigned char bytes[4];
  int arr[3];
  long tail;
};

// The value of type T whose bytes are all v.
#define REP(T, v) ((T)(0x0101010101010101ULL * (unsigned char)(v)))

static void set_bytes(unsigned char *p, size_t n, unsigned char v) {
  for (size_t i = 0; i < n; i++)
    p[i] = v;
}

static int all_bytes(const unsigned char *p, size_t n, unsigned char v) {
  for (size_t i = 0; i < n; i++)
    if (p[i] != v)
      return 0;
  return 1;
}

// Checks that field F of type T, at byte OFF of *s, is written through, in
// both directions,
// by the bytes of the field and by the bytes of the whole struct:
//  - bytes of the field -> field, and field -> bytes of the field,
//  - bytes of the struct -> field, and field -> bytes of the struct.
// Leaves the field with all bytes equal to v + 3.
#define CHECK_FIELD(s, F, T, OFF, v)                                           \
  do {                                                                         \
    unsigned char *fb = (unsigned char *)&(s)->F;                              \
    unsigned char *sb = (unsigned char *)(s) + (OFF);                          \
    set_bytes(fb, sizeof(T), (v));                                             \
    assert((s)->F == REP(T, (v)));                                             \
    assert(all_bytes(sb, sizeof(T), (v)));                                     \
    (s)->F = REP(T, (v) + 1);                                                  \
    assert(all_bytes(fb, sizeof(T), (v) + 1));                                 \
    assert(all_bytes(sb, sizeof(T), (v) + 1));                                 \
    set_bytes(sb, sizeof(T), (v) + 2);                                         \
    assert((s)->F == REP(T, (v) + 2));                                         \
    assert(all_bytes(fb, sizeof(T), (v) + 2));                                 \
    (s)->F = REP(T, (v) + 3);                                                  \
    assert(all_bytes(sb, sizeof(T), (v) + 3));                                 \
    assert(all_bytes(fb, sizeof(T), (v) + 3));                                 \
  } while (0)

// Checks that all fields hold the values CHECK_FIELD left in them, i.e., that
// no write through bytes clobbered a sibling field.
static void check_final(struct S *s) {
  assert(s->x == REP(int, 0x13));
  assert(s->in.a == REP(short, 0x23));
  assert(s->in.b == REP(int, 0x33));
  assert(s->bytes[0] == 0x43 && s->bytes[1] == 0x53);
  assert(s->bytes[2] == 0x63 && s->bytes[3] == 0x73);
  assert(s->arr[0] == REP(int, 0x83) && s->arr[1] == REP(int, 0x93));
  assert(s->arr[2] == REP(int, 0x0b));
  assert(s->tail == REP(long, 0x1b));
}

#define IN_OFF offsetof(struct S, in)
#define BYTES_OFF offsetof(struct S, bytes)
#define ARR_OFF offsetof(struct S, arr)

static void check_struct(struct S *s) {
  // Scalar fields, fields of a nested struct, and array elements.
  CHECK_FIELD(s, x, int, offsetof(struct S, x), 0x10);
  CHECK_FIELD(s, in.a, short, IN_OFF + offsetof(struct In, a), 0x20);
  CHECK_FIELD(s, in.b, int, IN_OFF + offsetof(struct In, b), 0x30);
  CHECK_FIELD(s, bytes[0], unsigned char, BYTES_OFF + 0, 0x40);
  CHECK_FIELD(s, bytes[1], unsigned char, BYTES_OFF + 1, 0x50);
  CHECK_FIELD(s, bytes[2], unsigned char, BYTES_OFF + 2, 0x60);
  CHECK_FIELD(s, bytes[3], unsigned char, BYTES_OFF + 3, 0x70);
  CHECK_FIELD(s, arr[0], int, ARR_OFF + 0 * sizeof(int), 0x80);
  CHECK_FIELD(s, arr[1], int, ARR_OFF + 1 * sizeof(int), 0x90);
  CHECK_FIELD(s, arr[2], int, ARR_OFF + 2 * sizeof(int), 0x08);
  CHECK_FIELD(s, tail, long, offsetof(struct S, tail), 0x18);
  check_final(s);

  // A nested struct as a whole, through its bytes.
  set_bytes((unsigned char *)&s->in, sizeof(struct In), 0x21);
  assert(s->in.a == REP(short, 0x21) && s->in.b == REP(int, 0x21));
  struct In in = {REP(short, 0x23), REP(int, 0x33)};
  s->in = in;
  assert(all_bytes((unsigned char *)&s->in.a, sizeof(short), 0x23));
  assert(all_bytes((unsigned char *)&s->in.b, sizeof(int), 0x33));

  // Array fields as a whole, through their bytes and through a different
  // element type.
  set_bytes((unsigned char *)s->arr, sizeof(s->arr), 0x85);
  assert(s->arr[0] == REP(int, 0x85) && s->arr[2] == REP(int, 0x85));
  set_bytes((unsigned char *)&s->arr, sizeof(s->arr), 0x86);
  assert(s->arr[1] == REP(int, 0x86));
  s->arr[0] = REP(int, 0x83);
  s->arr[1] = REP(int, 0x93);
  s->arr[2] = REP(int, 0x0b);
  int *ib = (int *)s->bytes;
  *ib = REP(int, 0x45);
  assert(all_bytes(s->bytes, sizeof(s->bytes), 0x45));
  s->bytes[0] = 0x43;
  s->bytes[1] = 0x53;
  s->bytes[2] = 0x63;
  s->bytes[3] = 0x73;
  assert(*ib == *(int *)(unsigned char *)&s->bytes);

  // Through void *, memcpy and memset.
  void *v = &s->in.b;
  *(int *)v = 11;
  assert(s->in.b == 11);
  int twelve = 12;
  memcpy(&s->x, &twelve, sizeof(int));
  assert(s->x == 12);
  memset(&s->in.b, 0x33, sizeof(int));
  memset(&s->x, 0x13, sizeof(int));
  check_final(s);
}

int main(void) {
  // A struct on the stack.
  struct S local;
  memset(&local, 0, sizeof(local));
  check_struct(&local);

  // A struct in an array.
  struct S arr[2];
  memset(arr, 0, sizeof(arr));
  check_struct(&arr[1]);
  assert(all_bytes((unsigned char *)&arr[0], sizeof(struct S), 0));

  // A struct in malloc'ed memory, i.e., a view of a byte buffer.
  struct S *heap = calloc(1, sizeof(struct S));
  assert(heap != NULL);
  check_struct(heap);
  free(heap);

  // A struct in a byte buffer that is written through the buffer.
  unsigned char *raw = malloc(sizeof(struct S));
  assert(raw != NULL);
  memset(raw, 0, sizeof(struct S));
  struct S *view = (struct S *)raw;
  check_struct(view);
  assert(raw[offsetof(struct S, tail)] == 0x1b);
  set_bytes(raw + offsetof(struct S, arr), sizeof(int), 0x77);
  assert(view->arr[0] == REP(int, 0x77));
  view->bytes[3] = 0x11;
  assert(raw[offsetof(struct S, bytes) + 3] == 0x11);
  free(raw);
  return 0;
}
