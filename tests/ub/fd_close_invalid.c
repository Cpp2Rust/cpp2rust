// panic-ub: refcount
#include <unistd.h>

int main(void) { return close(1234) == -1 ? 0 : 1; }
