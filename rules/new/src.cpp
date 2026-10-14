// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <new>

void *f1(std::size_t a0) { return ::operator new(a0); }

void f2(void *a0) { return ::operator delete(a0); }

void *f3(std::size_t a0) { return ::operator new[](a0); }

void f4(void *a0) { return ::operator delete[](a0); }
