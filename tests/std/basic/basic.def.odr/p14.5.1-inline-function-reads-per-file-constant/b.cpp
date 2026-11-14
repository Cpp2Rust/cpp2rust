#include "h.h"

const int *b_factor_address() { return &factor; }

int (*b_scaled_address())(int) { return &scaled; }

int b_scaled(int v) { return scaled(v); }
