// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

bool f9(long a, long b, long *r) { return __builtin_mul_overflow(a, b, r); }

bool f10(long long a, long long b, long long *r) { return __builtin_mul_overflow(a, b, r); }

bool f19() { return __builtin_is_constant_evaluated(); }
