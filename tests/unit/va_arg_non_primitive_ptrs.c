#include <assert.h>
#include <stdarg.h>
#include <stdio.h>

struct node {
  int data;
  struct node *next;
};

enum opt {
  OPT_STRING_OUT,
  OPT_FILE,
  OPT_NODE,
  OPT_NODE_OUT,
};

int dispatch(int option, ...) {
  va_list ap;
  va_start(ap, option);
  int result = 0;
  switch (option) {
  case OPT_STRING_OUT: {
    const char **out = va_arg(ap, const char **);
    *out = "hello";
    result = 1;
    break;
  }
  case OPT_FILE: {
    FILE *f = va_arg(ap, FILE *);
    result = (f != NULL);
    break;
  }
  case OPT_NODE: {
    struct node *n = va_arg(ap, struct node *);
    result = n->data;
    break;
  }
  case OPT_NODE_OUT: {
    struct node **out = va_arg(ap, struct node **);
    *out = NULL;
    result = 2;
    break;
  }
  }
  va_end(ap);
  return result;
}

int main() {
  const char *s = NULL;
  assert(dispatch(OPT_STRING_OUT, &s) == 1);
  assert(s != NULL);

  assert(dispatch(OPT_FILE, stdout) == 1);
  assert(dispatch(OPT_FILE, (FILE *)NULL) == 0);

  struct node head = {42, NULL};
  assert(dispatch(OPT_NODE, &head) == 42);

  struct node *outp = &head;
  assert(dispatch(OPT_NODE_OUT, &outp) == 2);
  assert(outp == NULL);

  return 0;
}
