// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1<T1: UnsafeIterator>() -> T1 {
    Default::default()
}

fn t2() -> isize {
    Default::default()
}

fn f1<T1: UnsafeIterator>() -> T1 {
    Default::default()
}

fn f2<T1>(a0: T1) -> T1 {
    a0
}

fn f3<T1>(a0: T1) -> T1 {
    a0
}

fn f4<T1: UnsafeIterator, T2>(a0: T1) -> T1 {
    a0.offset(-1)
}

fn f5<T1: UnsafeIterator, T2>(a0: T1) -> T1 {
    a0.offset(-1)
}

unsafe fn f6<T1: UnsafeIterator>(mut a0: T1) -> T1 {
    a0.prefix_dec()
}

unsafe fn f7<T1: UnsafeIterator>(mut a0: T1) -> T1 {
    a0.postfix_dec()
}

fn f8<T1: UnsafeIterator>(a0: T1, a1: isize) -> T1 {
    a0.offset(Into::<isize>::into(-a1))
}

fn f9<T1: UnsafeIterator>(a0: T1, a1: isize) -> T1 {
    a0.offset(Into::<isize>::into(a1))
}

fn f10<T1: UnsafeIterator>(a0: T1, a1: T1) -> bool {
    a0 == a1
}
