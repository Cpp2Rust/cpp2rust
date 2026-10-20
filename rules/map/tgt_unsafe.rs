// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1<T1, T2>() -> UnsafeMap<T1, (T1, T2)> {
    UnsafeMap::new()
}

fn t4() -> usize {
    0_usize
}

fn t5<T1>() -> T1 {
    unimplemented!()
}

fn t6<T1, T2>() -> T2 {
    unimplemented!()
}

unsafe fn f1<T1: Ord + Clone, T2: Default>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: T1) -> &mut T2 {
    &mut (*a0.get_or_insert_with(a1.clone(), || (a1, <T2>::default()))).1
}

unsafe fn f2<T1, T2>(a0: UnsafeMap<T1, (T1, T2)>) -> usize {
    a0.len()
}

unsafe fn f3<T1: Ord + Clone, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: UnsafeBidirectionalIterator<(T1, T2)>) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.erase(&a1)
}

unsafe fn f5<T1, T2>() -> UnsafeMap<T1, (T1, T2)> {
    UnsafeMap::new()
}

unsafe fn f6<T1: Ord + Clone, T2: Clone>(a0: UnsafeMap<T1, (T1, T2)>) -> UnsafeMap<T1, (T1, T2)> {
    a0.clone()
}

unsafe fn f7<T1: Ord, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: T1) -> *const T2 {
    &raw const (*a0.get(&a1).expect("out of range!")).1
}

unsafe fn f8<T1: Ord + Clone, T2: Default>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: T1) -> &mut T2 {
    &mut (*a0.get_or_insert_with(a1.clone(), || (a1, <T2>::default()))).1
}

unsafe fn f9<T1, T2>(a0: UnsafeMap<T1, (T1, T2)>) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.end()
}

unsafe fn f10<T1: Ord, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: T1) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.find(&a1)
}

unsafe fn f12<T1, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.begin()
}

unsafe fn f14<T1, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.end()
}

unsafe fn f15<T1: Ord, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: T1) -> *const T2 {
    &raw const (*a0.get(&a1).expect("out of range!")).1
}

unsafe fn f17<T1: Ord, T2>(a0: UnsafeMap<T1, (T1, T2)>, a1: T1) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.find(&a1)
}

unsafe fn f24<T1, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>) -> UnsafeMap<T1, (T1, T2)> {
    std::mem::take(&mut *a0)
}

unsafe fn f25<T1, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: &mut UnsafeMap<T1, (T1, T2)>) {
    *a0 = std::mem::take(&mut *a1)
}

unsafe fn f26<T1: Ord + Clone, T2: Clone>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: UnsafeMap<T1, (T1, T2)>) {
    *a0 = a1.clone()
}

unsafe fn f27<T1: Ord + Clone, T2>(
    a0: &mut UnsafeMap<T1, (T1, T2)>,
    a1: UnsafeBidirectionalIterator<(T1, T2)>,
) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.erase(&a1)
}

unsafe fn f28<T1: Ord, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: T1) -> usize {
    a0.erase_key(&a1)
}

unsafe fn f29<T1, T2>(a0: UnsafeMap<T1, (T1, T2)>) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.begin()
}
