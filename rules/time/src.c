// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <sys/time.h>
#include <time.h>

time_t f1(time_t *t) { return time(t); }

int f2(clockid_t clk_id, struct timespec *tp) {
  return clock_gettime(clk_id, tp);
}

int f3(struct timeval *tv, struct timezone *tz) {
  return gettimeofday(tv, tz);
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
