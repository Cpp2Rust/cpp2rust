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

static void print_tm(time_t t) {
  struct tm tm;
  assert(gmtime_r(&t, &tm) != NULL);
  printf("%d-%d-%d %d:%d:%d wday=%d yday=%d %s gmtoff=%ld isdst=%d\n",
         tm.tm_year, tm.tm_mon, tm.tm_mday, tm.tm_hour, tm.tm_min, tm.tm_sec,
         tm.tm_wday, tm.tm_yday, tm.tm_zone, tm.tm_gmtoff, tm.tm_isdst);
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

int main(void) {
  test_time();
  test_gmtime_r();
  test_strftime();
  return 0;
}
