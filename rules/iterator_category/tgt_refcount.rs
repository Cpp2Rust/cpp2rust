// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1<T1>() -> InputIterator<T1> {
    InputIterator::uninitialized()
}

fn t2<T1>() -> OutputIterator<T1> {
    OutputIterator::uninitialized()
}

fn t3<T1>() -> ForwardIterator<T1> {
    ForwardIterator::uninitialized()
}

fn t4<T1>() -> BidirectionalIterator<T1> {
    BidirectionalIterator::uninitialized()
}

fn t5<T1>() -> RandomAccessIterator<T1> {
    RandomAccessIterator::default()
}

fn f1<T1: CppIterator<T2>, T2>(a0: T1) -> T1 {
    a0.clone()
}

fn f2<T1: CppIterator<T2>, T2>(a0: T1) -> T1 {
    a0
}

fn f3<T1: CppIterator<T2>, T2>(a0: &mut T1, a1: T1) {
    *a0 = a1.clone()
}

fn f4<T1: CppIterator<T2>, T2>(a0: &mut T1, a1: T1) {
    *a0 = a1
}

fn f5<T1: CppIterator<T2>, T2>(a0: T1) -> Ptr<T2> {
    a0.iter_deref()
}

fn f6<T1: CppIterator<T2>, T2>(a0: &mut T1) -> T1 {
    a0.iter_inc()
}

fn f7<T1: CppIterator<T2>, T2>(a0: &mut T1) -> T1 {
    a0.iter_post_inc()
}

fn f8<T1: CppInputIterator<T2>, T2>(a0: T1) -> Ptr<T2> {
    a0.iter_deref()
}

fn f9<T1: CppInputIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 == a1
}

fn f10<T1: CppInputIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 != a1
}

fn f11<T1: CppBidirectionalIterator<T2>, T2>(a0: &mut T1) -> T1 {
    a0.iter_dec()
}

fn f12<T1: CppBidirectionalIterator<T2>, T2>(a0: &mut T1) -> T1 {
    a0.iter_post_dec()
}

fn f13<T1: CppRandomAccessIterator<T2>, T2>(a0: &mut T1, a1: isize) -> T1 {
    a0.iter_add_assign(a1)
}

fn f14<T1: CppRandomAccessIterator<T2>, T2>(a0: &mut T1, a1: isize) -> T1 {
    a0.iter_add_assign(-a1)
}

fn f15<T1: CppRandomAccessIterator<T2>, T2>(a0: T1, a1: isize) -> T1 {
    a0.iter_add(a1)
}

fn f16<T1: CppRandomAccessIterator<T2>, T2>(a0: isize, a1: T1) -> T1 {
    a1.iter_add(a0)
}

fn f17<T1: CppRandomAccessIterator<T2>, T2>(a0: T1, a1: isize) -> T1 {
    a0.iter_add(-a1)
}

fn f18<T1: CppRandomAccessIterator<T2>, T2>(a0: T1, a1: T1) -> isize {
    a0.iter_distance(&a1)
}

fn f19<T1: CppRandomAccessIterator<T2>, T2>(a0: T1, a1: isize) -> Ptr<T2> {
    a0.iter_at(a1)
}

fn f20<T1: CppRandomAccessIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 < a1
}

fn f21<T1: CppRandomAccessIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 > a1
}

fn f22<T1: CppRandomAccessIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 <= a1
}

fn f23<T1: CppRandomAccessIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 >= a1
}

fn f24<T1: From<T2>, T2>(a0: T2) -> T1 {
    T1::from(a0)
}
