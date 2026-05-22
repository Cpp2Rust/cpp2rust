#pragma once

/* Forward declaration only; no full definition exists anywhere in the build. */
struct opaque;

struct container {
  struct opaque *p;
  int x;
};

void touch(struct container *c);
