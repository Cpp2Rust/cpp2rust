#include <assert.h>
#include <stdio.h>
#include <sys/time.h>
#include <time.h>
#include <unistd.h>

static void test_time(void) {
  time_t t1 = time(NULL);
  time_t t2 = 0;
  time_t t3 = time(&t2);
  assert(t1 > 1500000000);
  assert(t2 == t3);
  assert(t3 >= t1);
}

static void test_clock_gettime(void) {
  struct timespec ts;
  assert(clock_gettime(CLOCK_REALTIME, &ts) == 0);
  assert(ts.tv_sec > 1500000000);
  assert(ts.tv_nsec >= 0 && ts.tv_nsec < 1000000000);
}

static void print_tm(time_t t) {
  struct tm tm;
  assert(gmtime_r(&t, &tm) != NULL);
  printf("%d-%d-%d %d:%d:%d wday=%d yday=%d\n", tm.tm_year, tm.tm_mon,
         tm.tm_mday, tm.tm_hour, tm.tm_min, tm.tm_sec, tm.tm_wday, tm.tm_yday);
}

static void test_gmtime_r(void) {
  print_tm(0);
  print_tm(1);
  print_tm(86399);
  print_tm(86400);
  print_tm(951782400); /* leap day */
  print_tm(951868799);
  print_tm(1704067199); /* year boundary */
  print_tm(1704067200);
  print_tm(1721126096);
  print_tm(4102444800);
}

static void test_strftime(void) {
  time_t t = 1721126096;
  struct tm tm;
  assert(gmtime_r(&t, &tm) != NULL);
  char buf[64];
  assert(strftime(buf, sizeof(buf), "%Y-%m-%d %H:%M:%S", &tm) > 0);
  printf("%s\n", buf);
  assert(strftime(buf, sizeof(buf), "%a, %d %b %Y %T", &tm) > 0);
  printf("%s\n", buf);
  assert(strftime(buf, sizeof(buf), "day %j 100%%", &tm) > 0);
  printf("%s\n", buf);
  assert(strftime(buf, sizeof(buf), "%e", &tm) > 0);
  printf("%s\n", buf);
  char small[4];
  assert(strftime(small, sizeof(small), "%Y-%m-%d", &tm) == 0);
}

static void test_gettimeofday(void) {
  struct timeval tv;
  assert(gettimeofday(&tv, NULL) == 0);
  assert(tv.tv_sec > 1500000000);
  assert(tv.tv_usec >= 0 && tv.tv_usec < 1000000);
}

static void test_utimes(void) {
  const char *path = "/tmp/cpp2rust_utimes_test.tmp";
  FILE *fp = fopen(path, "wb");
  assert(fp != NULL);
  assert(fclose(fp) == 0);
  struct timeval times[2];
  times[0].tv_sec = 1000000000;
  times[0].tv_usec = 0;
  times[1].tv_sec = 1000000001;
  times[1].tv_usec = 0;
  assert(utimes(path, times) == 0);
  assert(utimes("/tmp/cpp2rust_utimes_test_missing.tmp", times) == -1);
  assert(unlink(path) == 0);
}

int main(void) {
  test_time();
  test_clock_gettime();
  test_gmtime_r();
  test_strftime();
  test_gettimeofday();
  test_utimes();
  return 0;
}
