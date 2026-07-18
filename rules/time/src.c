// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <sys/time.h>
#include <time.h>

typedef struct tm t1;
typedef struct timeval t2;
typedef struct timespec t3;

time_t f1(time_t *t) { return time(t); }

int f2(clockid_t clk_id, struct timespec *tp) {
  return clock_gettime(clk_id, tp);
}

struct tm *f4(const time_t *timer, struct tm *result) {
  return gmtime_r(timer, result);
}

struct tm *f5(const time_t *timer, struct tm *result) {
  return localtime_r(timer, result);
}

size_t f6(char *s, size_t maxsize, const char *format, const struct tm *tp) {
  return strftime(s, maxsize, format, tp);
}

int f7(const char *file, const struct timeval tvp[2]) {
  return utimes(file, tvp);
}

#if defined(__linux__)
int f8(struct timeval *tv, struct timezone *tz) {
  return gettimeofday(tv, tz);
}
#elif defined(__APPLE__)
int f8(struct timeval *tv, void *tz) {
  return gettimeofday(tv, tz);
}
#else
#error "Unsupported platform for gettimeofday"
#endif

clockid_t f9() {
  return CLOCK_REALTIME;
}
