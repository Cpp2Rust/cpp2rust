// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t5<T1>() -> Vec<T1> {
    Default::default()
}

fn t9() -> usize {
    0_usize
}

fn t10<T1>() -> T1 {
    unimplemented!()
}

fn t11<T1>() -> Vec<T1> {
    Vec::new()
}

fn t12() -> bool {
    false
}

fn t13<T1>() -> *mut T1 {
    ::std::ptr::null_mut()
}

fn t14<T1>() -> *const T1 {
    ::std::ptr::null()
}

unsafe fn f38(a0: usize, a1: bool) -> Vec<bool> {
    (0..(a0) as usize).map(|_| a1).collect::<Vec<bool>>()
}

unsafe fn f40<T1>(a0: Vec<T1>) -> *const T1 {
    a0.as_ptr().add(a0.len())
}

unsafe fn f47(a0: Vec<bool>) -> Vec<bool> {
    a0
}

unsafe fn f60<T1>(a0: &mut Vec<T1>, a1: UnsafeRandomAccessIterator<T1>) -> UnsafeRandomAccessIterator<T1> {
    let pos = a1.iter_distance(&UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr())) as usize;
    a0.remove(pos);
    a1
}

unsafe fn f61<T1>(a0: Vec<T1>) -> usize {
    a0.len()
}

unsafe fn f62<T1>(a0: Vec<T1>) -> bool {
    a0.is_empty()
}

unsafe fn f63<T1>() -> Vec<T1> {
    Vec::new()
}

unsafe fn f64<T1>(a0: &mut Vec<T1>) {
    a0.pop();
}

unsafe fn f65<T1>(a0: &mut Vec<T1>) -> *mut T1 {
    a0.as_mut_ptr()
}

unsafe fn f66<T1>(a0: &mut Vec<T1>, a1: usize) -> *mut T1 {
    (&mut (a0)[a1 as usize] as *mut T1)
}

unsafe fn f67<T1: Default>(a0: usize) -> Vec<T1> {
    (0..(a0) as usize)
        .map(|_| <T1>::default())
        .collect::<Vec<_>>()
}

unsafe fn f68<T1>(a0: &mut Vec<T1>) -> *mut T1 {
    ((a0).first_mut().unwrap())
}

unsafe fn f69<T1>(a0: &mut Vec<T1>) -> *mut T1 {
    ((a0).last_mut().unwrap())
}

unsafe fn f70<T1>(a0: Vec<T1>) -> usize {
    a0.capacity()
}

unsafe fn f71<T1>(a0: &mut Vec<T1>, a1: usize) {
    if a1 as usize > a0.capacity() as usize {
        let len_0 = a0.len();
        a0.reserve_exact(a1 as usize - len_0 as usize);
    }
}

unsafe fn f72<T1>(a0: &mut Vec<T1>) -> UnsafeRandomAccessIterator<T1> {
    a0.begin()
}

unsafe fn f73<T1: Default>(a0: &mut Vec<T1>, a1: &mut T1) {
    let __a1 = std::mem::take(&mut *a1);
    a0.push(__a1)
}

unsafe fn f74<T1: Default>(a0: &mut Vec<T1>, a1: usize) {
    let __a0 = a1 as usize;
    a0.resize_with(__a0, || <T1>::default())
}

unsafe fn f75<T1>(a0: &mut Vec<T1>) {
    a0.clear()
}

unsafe fn f76<T1>(a0: &mut Vec<T1>) -> UnsafeRandomAccessIterator<T1> {
    a0.end()
}

unsafe fn f77<T1>(a0: &mut Vec<T1>, a1: UnsafeRandomAccessIterator<T1>, a2: T1) -> UnsafeRandomAccessIterator<T1> {
    let pos = a1.iter_distance(&UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr())) as usize;
    a0.insert(pos, a2);
    a1
}

unsafe fn f78<T1: Default + Clone>(a0: usize, a1: T1) -> Vec<T1> {
    vec![a1; a0 as usize]
}

unsafe fn f79<T1>(a0: &mut Vec<T1>, a1: UnsafeRandomAccessIterator<T1>, a2: T1) -> UnsafeRandomAccessIterator<T1> {
    let pos = a1.iter_distance(&UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr())) as usize;
    a0.insert(pos, a2);
    a1
}

unsafe fn f80<T1: Clone>(a0: &mut Vec<T1>, a1: T1) {
    let a0_clone = a1.clone();
    a0.push(a0_clone)
}

unsafe fn f91<T1>(a0: Vec<T1>) -> Vec<T1> {
    a0
}

unsafe fn f92<T1: TryFrom<T2>, T2: Clone, T3: UnsafeCppInputIterator<T2>>(a0: T3, a1: T3) -> Vec<T1> {
    let mut __v = Vec::new();
    let mut __it = a0;
    while __it != a1 {
        __v.push(<T1>::try_from((*__it.iter_deref()).clone()).ok().unwrap());
        __it.iter_inc();
    }
    __v
}

unsafe fn f93<T1>(a0: &mut Vec<T1>) -> *const T1 {
    a0.as_ptr()
}

unsafe fn f95<T1>(a0: Vec<T1>) -> UnsafeRandomAccessIterator<T1> {
    a0.begin()
}

unsafe fn f96<T1>(a0: Vec<T1>) -> UnsafeRandomAccessIterator<T1> {
    a0.end()
}

unsafe fn f97(a0: &mut Vec<bool>, a1: &mut Vec<bool>) {
    std::mem::swap(&mut *a0, &mut *a1)
}

unsafe fn f98<T1: Copy>(a0: &mut Vec<T1>, a1: usize) -> *mut T1 {
    if a1 as usize >= a0.len() {
        panic!("out of bounds access")
    } else {
        (a0).as_mut_ptr().add(a1 as usize)
    }
}

unsafe fn f99<T1>(a0: &Vec<T1>) -> &T1 {
    ((a0).last().unwrap())
}

unsafe fn f101<T1: Clone, T2, T3: UnsafeCppInputIterator<T1>>(
    a0: &mut Vec<T1>,
    a1: UnsafeRandomAccessIterator<T1>,
    a2: T3,
    a3: T3,
) -> UnsafeRandomAccessIterator<T1> {
    let __off = a1.iter_distance(&UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr())) as usize;
    let mut __tmp = Vec::new();
    let mut __it = a2;
    while __it != a3 {
        __tmp.push((*__it.iter_deref()).clone());
        __it.iter_inc();
    }
    a0.splice(__off..__off, __tmp);
    UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr().add(__off))
}

unsafe fn f102<T1: Default + Clone>(a0: &mut Vec<T1>, a1: usize, a2: T1) {
    let __a0 = a1 as usize;
    a0.resize(__a0, a2)
}

unsafe fn f103<T1: Clone>(a0: &mut Vec<T1>, a1: &mut Vec<T1>) {
    *a0 = std::mem::take(&mut *a1)
}

unsafe fn f104<T1>(a0: Vec<T1>) -> UnsafeRandomAccessIterator<T1> {
    a0.end()
}

unsafe fn f105<T1: Clone>(a0: &mut Vec<T1>, a1: Vec<T1>) {
    *a0 = a1.clone()
}

unsafe fn f106<T1>(a0: &mut Vec<T1>) {
    a0.shrink_to_fit()
}

unsafe fn f108<T1>(a0: &mut Vec<T1>) -> Vec<T1> {
    std::mem::take(&mut *a0)
}

unsafe fn f110<T1: Clone>(a0: Vec<T1>) -> Vec<T1> {
    a0.clone()
}

unsafe fn f114<T1>(a0: &mut Vec<T1>, init: T1) {
    let __init = init;
    a0.push(__init)
}

unsafe fn f115<T1: Clone>(a0: &mut Vec<T1>, a1: UnsafeRandomAccessIterator<T1>, a2: usize, a3: T1) -> UnsafeRandomAccessIterator<T1> {
    let __off = a1.iter_distance(&UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr())) as usize;
    a0.splice(__off..__off, std::iter::repeat_n(a3, a2));
    UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr().add(__off))
}

unsafe fn f116<T1>(a0: &mut Vec<T1>, a1: UnsafeRandomAccessIterator<T1>, a2: Vec<T1>) -> UnsafeRandomAccessIterator<T1> {
    let __off = a1.iter_distance(&UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr())) as usize;
    a0.splice(__off..__off, a2);
    UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr().add(__off))
}
