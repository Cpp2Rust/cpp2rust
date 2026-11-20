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
    &mut (*a0.subscript(&a1))
}

fn f2<T1, T2>(a0: UnsafeMap<T1, (T1, T2)>) -> usize {
    a0.len()
}

fn f3<T1: Ord, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: UnsafeBidirectionalIterator<(T1, T2)>) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.erase(&a1)
}

fn f5<T1, T2>() -> UnsafeMap<T1, (T1, T2)> {
    UnsafeMap::new()
}

fn f6<T1: Ord + Clone, T2: Clone>(a0: UnsafeMap<T1, (T1, T2)>) -> UnsafeMap<T1, (T1, T2)> {
    a0.clone()
}

unsafe fn f7<T1: Ord, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: T1) -> *const T2 {
    &raw const (*a0.get(&a1).expect("out of range!")).1
}

unsafe fn f8<T1: Ord, T2: Default>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: *mut T1) -> &mut T2 {
    &mut (*a0.subscript_move(a1, |__k| cpp_move_unsafe!(__k)))
}

fn f9<T1, T2>(a0: UnsafeMap<T1, (T1, T2)>) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.end()
}

fn f10<T1: Ord, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: T1) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.find(&a1)
}

fn f12<T1, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.begin()
}

fn f14<T1, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.end()
}

unsafe fn f15<T1: Ord, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: T1) -> *const T2 {
    &raw const (*a0.get(&a1).expect("out of range!")).1
}

fn f17<T1: Ord, T2>(a0: UnsafeMap<T1, (T1, T2)>, a1: T1) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.find(&a1)
}

fn f24<T1, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>) -> UnsafeMap<T1, (T1, T2)> {
    std::mem::take(&mut *a0)
}

fn f25<'a, T1, T2>(a0: &'a mut UnsafeMap<T1, (T1, T2)>, a1: &mut UnsafeMap<T1, (T1, T2)>) -> &'a mut UnsafeMap<T1, (T1, T2)> {
    *a0 = std::mem::take(&mut *a1);
    a0
}

fn f26<T1: Ord + Clone, T2: Clone>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: UnsafeMap<T1, (T1, T2)>) -> &mut UnsafeMap<T1, (T1, T2)> {
    *a0 = a1.clone();
    a0
}

fn f27<T1: Ord, T2>(
    a0: &mut UnsafeMap<T1, (T1, T2)>,
    a1: UnsafeBidirectionalIterator<(T1, T2)>,
) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.erase(&a1)
}

fn f28<T1: Ord, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: T1) -> usize {
    a0.erase_key(&a1)
}

fn f29<T1, T2>(a0: UnsafeMap<T1, (T1, T2)>) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.begin()
}

fn f67<T1: Ord, T2>(
    a0: &mut UnsafeMap<T1, (T1, T2)>,
    init: cpp_type!("typename std::map<T1, T2, T3, T4>::value_type", (T1, T2)),
) -> (UnsafeBidirectionalIterator<(T1, T2)>, bool) {
    a0.emplace(init)
}

fn f68<T1: Ord, T2>(
    a0: &mut UnsafeMap<T1, (T1, T2)>,
    a1: UnsafeBidirectionalIterator<(T1, T2)>,
    init: cpp_type!("typename std::map<T1, T2, T3, T4>::value_type", (T1, T2)),
) -> UnsafeBidirectionalIterator<(T1, T2)> {
    let _ = a1;
    a0.emplace(init).0
}

fn f73<T1: Ord, T2>(
    a0: &mut UnsafeMap<T1, (T1, T2)>,
    a1: T1,
    init: T2,
) -> (UnsafeBidirectionalIterator<(T1, T2)>, bool) {
    a0.try_emplace(a1, || init)
}
