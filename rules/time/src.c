// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <sys/time.h>
#include <time.h>
#endif

typedef struct tm CPP2RUST_TYPE_RULE(1);
typedef struct timeval CPP2RUST_TYPE_RULE(2);
typedef struct timespec CPP2RUST_TYPE_RULE(3);

time_t CPP2RUST_EXPR_RULE(1)(time_t *t) { return time(t); }

int CPP2RUST_EXPR_RULE(2)(clockid_t clk_id, struct timespec *tp) {
  return clock_gettime(clk_id, tp);
}

struct tm *CPP2RUST_EXPR_RULE(4)(const time_t *timer, struct tm *result) {
  return gmtime_r(timer, result);
}

struct tm *CPP2RUST_EXPR_RULE(5)(const time_t *timer, struct tm *result) {
  return localtime_r(timer, result);
}

size_t CPP2RUST_EXPR_RULE(6)(char *s, size_t maxsize, const char *format,
                             const struct tm *tp) {
  return strftime(s, maxsize, format, tp);
}

int CPP2RUST_EXPR_RULE(7)(const char *file, const struct timeval tvp[2]) {
  return utimes(file, tvp);
}

#if defined(__linux__)
int CPP2RUST_EXPR_RULE(8)(struct timeval *tv, struct timezone *tz) {
  return gettimeofday(tv, tz);
}
#elif defined(__APPLE__)
int CPP2RUST_EXPR_RULE(8)(struct timeval *tv, void *tz) {
  return gettimeofday(tv, tz);
}
#else
#error "Unsupported platform for gettimeofday"
#endif

clockid_t CPP2RUST_EXPR_RULE(9)() { return CLOCK_REALTIME; }

clockid_t CPP2RUST_EXPR_RULE(10)() { return CLOCK_MONOTONIC; }

#ifdef __linux__
clockid_t CPP2RUST_EXPR_RULE(11)() { return CLOCK_MONOTONIC_RAW; }
#endif
