#include <assert.h>
#include <stdarg.h>

struct context {
  int verbose;
  int last_error;
};

void set_error(struct context *ctx, const char *fmt, ...) {
  if (ctx->verbose) {
    va_list ap;
    va_start(ap, fmt);
    ctx->last_error = va_arg(ap, int);
    va_end(ap);
  }
}

union value {
  int i;
  long l;
};

long pick(int use_long, ...) {
  va_list ap;
  va_start(ap, use_long);
  union value v = va_arg(ap, union value);
  va_end(ap);
  if (use_long) {
    return v.l;
  }
  return v.i;
}

int main() {
  struct context ctx;
  ctx.verbose = 1;
  ctx.last_error = 0;

  set_error(&ctx, "error %d", 42);
  assert(ctx.last_error == 42);

  ctx.verbose = 0;
  set_error(&ctx, "error %d", 99);
  assert(ctx.last_error == 42);

  union value v;
  v.l = 1L << 40;
  assert(pick(1, v) == 1L << 40);
  v.i = 7;
  assert(pick(0, v) == 7);

  return 0;
}
