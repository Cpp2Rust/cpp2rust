// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::{MapIterator, UnsafeMapIterator};
use std::collections::BTreeMap;

fn t1<T1, T2>() -> BTreeMap<T1, Box<T2>> {
    BTreeMap::new()
}

fn t2<T1: Clone + Ord, T2>() -> UnsafeMapIterator<T1, T2> {
    UnsafeMapIterator::null()
}

fn t3<T1: Clone + Ord, T2>() -> UnsafeMapIterator<T1, T2> {
    UnsafeMapIterator::null()
}

unsafe fn f1<T1: Ord + Clone, T2: Default>(a0: &mut BTreeMap<T1, Box<T2>>, a1: T1) -> &mut T2 {
    a0.entry(a1).or_default().as_mut()
}
unsafe fn f2<T1, T2>(a0: BTreeMap<T1, Box<T2>>) -> u64 {
    a0.len() as u64
}
unsafe fn f3<T1: Ord + Clone, T2>(
    a0: &mut BTreeMap<T1, Box<T2>>,
    a1: UnsafeMapIterator<T1, T2>,
) -> UnsafeMapIterator<T1, T2> {
    UnsafeMapIterator::erase(&*a0 as *const BTreeMap<T1, Box<T2>>, &a1)
}

unsafe fn f5<T1, T2>() -> BTreeMap<T1, Box<T2>> {
    BTreeMap::new()
}

unsafe fn f6<T1: Clone, T2: Clone>(a0: BTreeMap<T1, Box<T2>>) -> BTreeMap<T1, Box<T2>> {
    a0.clone()
}
unsafe fn f7<T1: Ord, T2>(a0: &mut BTreeMap<T1, Box<T2>>, a1: T1) -> *const T2 {
    (a0.get(&a1).expect("out of range!").as_ref() as *const T2)
}

unsafe fn f8<T1: Ord + Clone, T2: Default>(a0: &mut BTreeMap<T1, Box<T2>>, a1: T1) -> &mut T2 {
    a0.entry(a1).or_default().as_mut()
}
unsafe fn f9<T1: Ord + Clone, T2>(a0: BTreeMap<T1, Box<T2>>) -> UnsafeMapIterator<T1, T2> {
    UnsafeMapIterator::end(&a0 as *const BTreeMap<T1, Box<T2>>)
}
unsafe fn f10<T1: Ord + Clone, T2>(
    a0: &mut BTreeMap<T1, Box<T2>>,
    a1: T1,
) -> UnsafeMapIterator<T1, T2> {
    UnsafeMapIterator::find_key(&*a0 as *const BTreeMap<T1, Box<T2>>, &a1)
}
unsafe fn f11<T1: PartialEq, T2: PartialEq>(
    a0: UnsafeMapIterator<T1, T2>,
    a1: UnsafeMapIterator<T1, T2>,
) -> bool {
    a0 != a1
}
unsafe fn f12<T1: Ord + Clone, T2>(a0: &mut BTreeMap<T1, Box<T2>>) -> UnsafeMapIterator<T1, T2> {
    UnsafeMapIterator::begin(&*a0 as *const BTreeMap<T1, Box<T2>>)
}
unsafe fn f13<T1: PartialEq, T2: PartialEq>(
    a0: UnsafeMapIterator<T1, T2>,
    a1: UnsafeMapIterator<T1, T2>,
) -> bool {
    a0 == a1
}

unsafe fn f14<T1: Ord + Clone, T2>(a0: &mut BTreeMap<T1, Box<T2>>) -> UnsafeMapIterator<T1, T2> {
    UnsafeMapIterator::end(&*a0 as *const BTreeMap<T1, Box<T2>>)
}
unsafe fn f15<T1: Ord, T2>(a0: &mut BTreeMap<T1, Box<T2>>, a1: T1) -> *const T2 {
    (a0.get(&a1).expect("out of range!").as_ref() as *const T2)
}

unsafe fn f16<T1: PartialEq, T2: PartialEq>(
    a0: UnsafeMapIterator<T1, T2>,
    a1: UnsafeMapIterator<T1, T2>,
) -> bool {
    a0 == a1
}
unsafe fn f17<T1: Ord + Clone, T2>(a0: BTreeMap<T1, Box<T2>>, a1: T1) -> UnsafeMapIterator<T1, T2> {
    UnsafeMapIterator::find_key(&a0 as *const BTreeMap<T1, Box<T2>>, &a1)
}

unsafe fn f19<T1: Clone, T2>(a0: UnsafeMapIterator<T1, T2>) -> UnsafeMapIterator<T1, T2> {
    a0.clone()
}

unsafe fn f20<T1: Ord + Clone, T2>(a0: UnsafeMapIterator<T1, T2>) -> *const T1 {
    a0.first()
}
unsafe fn f21<T1: Ord + Clone, T2>(a0: UnsafeMapIterator<T1, T2>) -> *mut T2 {
    a0.second()
}

unsafe fn f22<T1: Ord + Clone, T2>(a0: UnsafeMapIterator<T1, T2>) -> *const T1 {
    a0.first()
}
unsafe fn f23<T1: Ord + Clone, T2>(a0: UnsafeMapIterator<T1, T2>) -> *mut T2 {
    a0.second()
}
