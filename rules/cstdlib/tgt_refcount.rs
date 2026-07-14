// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f2(a0: AnyPtr) {
    libcc2rs::free_refcount(a0)
}

fn f3(a0: usize) -> AnyPtr {
    libcc2rs::malloc_refcount(a0)
}

fn f4(a0: AnyPtr, a1: usize) -> AnyPtr {
    libcc2rs::realloc_refcount(a0, a1)
}

fn f5(a0: usize, a1: usize) -> AnyPtr {
    libcc2rs::calloc_refcount(a0, a1)
}
