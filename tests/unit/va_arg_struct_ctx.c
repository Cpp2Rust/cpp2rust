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

int main() {
  struct context ctx;
  ctx.verbose = 1;
  ctx.last_error = 0;

  set_error(&ctx, "error %d", 42);
  assert(ctx.last_error == 42);

  ctx.verbose = 0;
  set_error(&ctx, "error %d", 99);
  assert(ctx.last_error == 42);

  return 0;
}
