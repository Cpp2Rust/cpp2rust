// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1<T1>() -> UnsafeInputIterator<T1> {
    UnsafeInputIterator::uninitialized()
}

fn t2<T1>() -> UnsafeOutputIterator<T1> {
    UnsafeOutputIterator::uninitialized()
}

fn t3<T1>() -> UnsafeForwardIterator<T1> {
    UnsafeForwardIterator::uninitialized()
}

fn t4<T1>() -> UnsafeBidirectionalIterator<T1> {
    UnsafeBidirectionalIterator::uninitialized()
}

fn t5<T1>() -> UnsafeRandomAccessIterator<T1> {
    UnsafeRandomAccessIterator::default()
}

unsafe fn f1<T1: UnsafeCppIterator<T2>, T2>(a0: T1) -> T1 {
    a0.clone()
}

unsafe fn f2<T1: UnsafeCppIterator<T2>, T2>(a0: T1) -> T1 {
    a0
}

unsafe fn f3<T1: UnsafeCppIterator<T2>, T2>(a0: &mut T1, a1: T1) {
    *a0 = a1.clone()
}

unsafe fn f4<T1: UnsafeCppIterator<T2>, T2>(a0: &mut T1, a1: T1) {
    *a0 = a1
}

unsafe fn f5<T1: UnsafeCppIterator<T2>, T2>(a0: T1) -> *mut T2 {
    a0.deref()
}

unsafe fn f6<T1: UnsafeCppIterator<T2>, T2>(a0: &mut T1) -> T1 {
    a0.prefix_inc()
}

unsafe fn f7<T1: UnsafeCppIterator<T2>, T2>(a0: &mut T1) -> T1 {
    a0.postfix_inc()
}

unsafe fn f8<T1: UnsafeCppInputIterator<T2>, T2>(a0: T1) -> *mut T2 {
    a0.deref()
}

unsafe fn f9<T1: UnsafeCppInputIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 == a1
}

unsafe fn f10<T1: UnsafeCppInputIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 != a1
}

unsafe fn f11<T1: UnsafeCppBidirectionalIterator<T2>, T2>(a0: &mut T1) -> T1 {
    a0.prefix_dec()
}

unsafe fn f12<T1: UnsafeCppBidirectionalIterator<T2>, T2>(a0: &mut T1) -> T1 {
    a0.postfix_dec()
}

unsafe fn f13<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: &mut T1, a1: isize) -> T1 {
    a0.plus_assign(a1)
}

unsafe fn f14<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: &mut T1, a1: isize) -> T1 {
    a0.plus_assign(-a1)
}

unsafe fn f15<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: T1, a1: isize) -> T1 {
    a0.plus(a1)
}

unsafe fn f16<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: isize, a1: T1) -> T1 {
    a1.plus(a0)
}

unsafe fn f17<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: T1, a1: isize) -> T1 {
    a0.plus(-a1)
}

unsafe fn f18<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: T1, a1: T1) -> isize {
    a0.distance(&a1)
}

unsafe fn f19<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: T1, a1: isize) -> *mut T2 {
    a0.at(a1)
}

unsafe fn f20<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 < a1
}

unsafe fn f21<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 > a1
}

unsafe fn f22<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 <= a1
}

unsafe fn f23<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 >= a1
}
