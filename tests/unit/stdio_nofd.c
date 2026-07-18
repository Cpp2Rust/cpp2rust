#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

static void test_fputc_fputs(void) {
  const char *path = "/tmp/cpp2rust_stdio_nofd_puts.tmp";
  FILE *fp = fopen(path, "wb");
  assert(fp != NULL);
  assert(fputc('A', fp) == 'A');
  assert(fputs("BCD\n", fp) >= 0);
  assert(fclose(fp) == 0);
  fp = fopen(path, "rb");
  assert(fp != NULL);
  char buf[16] = {0};
  assert(fread(buf, 1, 16, fp) == 5);
  assert(memcmp(buf, "ABCD\n", 5) == 0);
  assert(fclose(fp) == 0);
  assert(unlink(path) == 0);
}

static void test_puts(void) { assert(puts("hello from puts") >= 0); }

static void test_fgets_getc(void) {
  const char *path = "/tmp/cpp2rust_stdio_nofd_gets.tmp";
  FILE *fp = fopen(path, "wb");
  assert(fp != NULL);
  assert(fputs("line1\nline2\n", fp) >= 0);
  assert(fclose(fp) == 0);
  fp = fopen(path, "rb");
  assert(fp != NULL);
  char buf[8];
  assert(fgets(buf, 8, fp) != NULL);
  assert(memcmp(buf, "line1\n", 7) == 0);
  assert(getc(fp) == 'l');
  assert(fgets(buf, 4, fp) != NULL);
  assert(memcmp(buf, "ine", 4) == 0);
  assert(fgets(buf, 8, fp) != NULL);
  assert(memcmp(buf, "2\n", 3) == 0);
  assert(fgets(buf, 8, fp) == NULL);
  assert(getc(fp) == EOF);
  assert(fclose(fp) == 0);
  assert(unlink(path) == 0);
}

static void test_freopen(void) {
  const char *path = "/tmp/cpp2rust_stdio_nofd_reopen.tmp";
  FILE *fp = fopen(path, "wb");
  assert(fp != NULL);
  assert(fputs("hello", fp) >= 0);
  FILE *fp2 = freopen(path, "rb", fp);
  assert(fp2 != NULL);
  char buf[8] = {0};
  assert(fread(buf, 1, 8, fp2) == 5);
  assert(memcmp(buf, "hello", 5) == 0);
  assert(fclose(fp2) == 0);
  assert(unlink(path) == 0);
}

static void test_fseeko(void) {
  const char *path = "/tmp/cpp2rust_stdio_nofd_seek.tmp";
  FILE *fp = fopen(path, "wb");
  assert(fp != NULL);
  assert(fputs("hello world", fp) >= 0);
  assert(fclose(fp) == 0);
  fp = fopen(path, "rb");
  assert(fp != NULL);
  assert(fseeko(fp, 6, SEEK_SET) == 0);
  char buf[8] = {0};
  assert(fread(buf, 1, 5, fp) == 5);
  assert(memcmp(buf, "world", 5) == 0);
  assert(fseeko(fp, -5, SEEK_END) == 0);
  assert(getc(fp) == 'w');
  assert(fseeko(fp, 1, SEEK_CUR) == 0);
  assert(getc(fp) == 'r');
  assert(fclose(fp) == 0);
  assert(unlink(path) == 0);
}

static void test_rename(void) {
  const char *from = "/tmp/cpp2rust_stdio_nofd_from.tmp";
  const char *to = "/tmp/cpp2rust_stdio_nofd_to.tmp";
  FILE *fp = fopen(from, "wb");
  assert(fp != NULL);
  assert(fputs("data", fp) >= 0);
  assert(fclose(fp) == 0);
  assert(rename(from, to) == 0);
  assert(fopen(from, "rb") == NULL);
  fp = fopen(to, "rb");
  assert(fp != NULL);
  assert(fclose(fp) == 0);
  assert(rename(from, to) == -1);
  assert(unlink(to) == 0);
}

static void test_setvbuf(void) {
  const char *path = "/tmp/cpp2rust_stdio_nofd_vbuf.tmp";
  FILE *fp = fopen(path, "wb");
  assert(fp != NULL);
  assert(setvbuf(fp, NULL, _IONBF, 0) == 0);
  assert(fputs("x", fp) >= 0);
  assert(fclose(fp) == 0);
  assert(unlink(path) == 0);
}

int main(void) {
  test_fputc_fputs();
  test_puts();
  test_fgets_getc();
  test_freopen();
  test_fseeko();
  test_rename();
  test_setvbuf();
  return 0;
}
