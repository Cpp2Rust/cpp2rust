// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1(a0: usize) -> AnyPtr {
    libcc2rs::malloc_refcount(a0)
}

fn f2(a0: AnyPtr) {
    libcc2rs::free_refcount(a0)
}

fn f3(a0: usize) -> AnyPtr {
    libcc2rs::malloc_refcount(a0)
}

fn f4(a0: AnyPtr) {
    libcc2rs::free_refcount(a0)
}

fn f5<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}
