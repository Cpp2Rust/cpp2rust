// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

struct T1;

fn types() {
    let t1: Vec<T1> = Default::default();
    let t2: *mut T1 = Default::default();
    let t3: Vec<Vec<T1>> = Vec::new();
    let t4: *const T1 = Default::default();
}

unsafe fn f1<T1>(a0: &mut Vec<T1>, a1: *const T1) -> *const T1 {
    let pos = a1.offset_from(a0.as_ptr()) as usize;
    a0.remove(pos);
    a1
}
// TODO: this should return usize. However std::size_t is translated as unsigned long which in turn
// gets translated to u64.
unsafe fn f2<T1>(a0: Vec<T1>) -> u64 {
    a0.len() as u64
}
unsafe fn f3<T1>(a0: Vec<T1>) -> bool {
    a0.is_empty()
}
unsafe fn f4<T1>() -> Vec<T1> {
    Vec::new()
}
unsafe fn f5<T1>(a0: &mut Vec<T1>) {
    a0.pop();
}
unsafe fn f6<T1>(a0: &mut Vec<T1>) -> *mut T1 {
    a0.as_mut_ptr()
}
unsafe fn f7<T1>(a0: &mut Vec<T1>, a1: usize) -> *mut T1 {
    &mut (a0)[a1 as usize]
}
unsafe fn f8<T1: Default>(a0: usize) -> Vec<T1> {
    (0..(a0) as usize)
        .map(|_| <T1>::default())
        .collect::<Vec<_>>()
}
unsafe fn f9<T1>(a0: &mut Vec<T1>) -> *mut T1 {
    ((a0).first_mut().unwrap())
}
unsafe fn f10<T1>(a0: &mut Vec<T1>) -> *mut T1 {
    ((a0).last_mut().unwrap())
}
unsafe fn f11<T1>(a0: Vec<T1>) -> u64 {
    a0.capacity() as u64
}
unsafe fn f12<T1>(a0: &mut Vec<T1>, a1: usize) {
    if a1 as usize > a0.capacity() as usize {
        let len_0 = a0.len();
        a0.reserve_exact(a1 as usize - len_0 as usize);
    }
}
unsafe fn f13<T1>(a0: &mut Vec<T1>) -> *mut T1 {
    a0.as_mut_ptr()
}
unsafe fn f14<T1: Default>(a0: &mut Vec<T1>, a1: &mut T1) {
    a0.push(std::mem::take(&mut *a1))
}
unsafe fn f15<T1: Default>(a0: &mut Vec<T1>, a1: usize) {
    let __a0 = a1 as usize;
    a0.resize_with(__a0, || <T1>::default())
}
unsafe fn f16<T1>(a0: &mut Vec<T1>) {
    a0.clear()
}
unsafe fn f17<T1>(a0: &mut Vec<T1>) -> *mut T1 {
    a0.as_mut_ptr().add(a0.len())
}
unsafe fn f18<T1>(a0: &mut Vec<T1>, a1: *const T1, a2: T1) {
    let pos = a1.offset_from(a0.as_ptr()) as usize;
    a0.insert(pos, a2);
}
unsafe fn f19<T1: Default + Clone>(a0: usize, a1: T1) -> Vec<T1> {
    vec![a1; a0 as usize]
}
unsafe fn f20<T1>(a0: &mut Vec<T1>, a1: *const T1, a2: T1) {
    let pos = a1.offset_from(a0.as_ptr()) as usize;
    a0.insert(pos, a2);
}
unsafe fn f21<T1: Clone>(a0: &mut Vec<T1>, a1: T1) {
    let a0_clone = a1.clone();
    a0.push(a0_clone)
}
unsafe fn f22<T1>(a0: *mut T1) -> *mut T1 {
    a0
}
unsafe fn f23<T1>(a0: *const T1) -> *const T1 {
    a0
}
unsafe fn f24<T1>(a0: *const T1) -> *const T1 {
    a0
}
unsafe fn f25<T1>(a0: *mut T1, a1: usize) -> *mut T1 {
    a0.add(a1 as usize)
}
unsafe fn f26<T1>(a0: *const T1, a1: *const T1) -> bool {
    a0 != a1
}
unsafe fn f27<T1>(a0: *const T1, a1: *const T1) -> bool {
    a0 == a1
}

unsafe fn f28<T1>(a0: &mut *mut T1) -> *mut T1 {
    a0.postfix_inc()
}
unsafe fn f29<T1: Clone>(a0: Vec<Vec<T1>>) -> Vec<Vec<T1>> {
    a0.clone()
}

unsafe fn f30<T1: Default + Clone>(a0: usize) -> Vec<Vec<T1>> {
    (0..(a0) as usize)
        .map(|_| <Vec<T1>>::default())
        .collect::<Vec<_>>()
}
unsafe fn f31<T1>(a0: &mut Vec<Vec<T1>>, a1: Vec<T1>) {
    a0.push(a1)
}
unsafe fn f32<T1: Default>(a0: &mut Vec<Vec<T1>>, a1: usize) {
    a0.resize_with(a1 as usize, || <Vec<T1>>::default())
}

unsafe fn f33<T1>(a0: *const T1, a1: *const T1) -> isize {
    a0.offset_from(a1)
}

unsafe fn f34<T1>(a0: &mut *mut T1) -> *mut T1 {
    a0.prefix_inc()
}

unsafe fn f35<T1: Clone>(a0: *const T1, a1: *const T1) -> Vec<T1> {
    core::slice::from_raw_parts(a0, (a1).offset_from(a0) as usize).to_vec()
}

unsafe fn f36<T1>(a0: Vec<T1>) -> Vec<T1> {
    a0
}

unsafe fn f37<T1: TryFrom<T2>, T2: Clone>(a0: *mut T2, a1: *mut T2) -> Vec<T1> {
    core::slice::from_raw_parts(a0, (a1).offset_from(a0) as usize)
        .iter()
        .map(|x| T1::try_from(x.clone()).ok().unwrap())
        .collect()
}

unsafe fn f38(a0: usize, a1: bool) -> Vec<bool> {
    (0..(a0) as usize).map(|_| a1).collect::<Vec<bool>>()
}

unsafe fn f40<T1>(a0: Vec<T1>) -> *const T1 {
    a0.as_ptr().add(a0.len())
}

unsafe fn f41<T1>(a0: &mut Vec<T1>) -> *const T1 {
    a0.as_ptr()
}

unsafe fn f42<T1: Ord>(a0: *const T1, a1: *const T1) -> *const T1 {
    core::slice::from_raw_parts(a0, (a1).offset_from(a0) as usize)
        .iter()
        .max()
        .unwrap()
}

unsafe fn f43<T1>(a0: Vec<T1>) -> *const T1 {
    a0.as_ptr()
}

unsafe fn f44<T1>(a0: Vec<T1>) -> *const T1 {
    a0.as_ptr().add(a0.len())
}

unsafe fn f47(a0: Vec<bool>) -> Vec<bool> {
    a0
}

unsafe fn f48(a0: &mut Vec<bool>, a1: &mut Vec<bool>) {
    std::mem::swap(&mut *a0, &mut *a1)
}

unsafe fn f50<T1: Copy>(a0: &mut Vec<T1>, a1: usize) -> *mut T1 {
    if a1 as usize >= a0.len() {
        panic!("out of bounds access")
    } else {
        (a0).as_mut_ptr().add(a1 as usize)
    }
}

unsafe fn f51<T1>(a0: &Vec<T1>) -> &T1 {
    ((a0).last().unwrap())
}

unsafe fn f52<T1: Clone>(a0: &mut Vec<Vec<T1>>, a1: Vec<T1>) {
    a0.push(a1.clone())
}

unsafe fn f53<T1: Clone>(a0: &mut Vec<T1>, a1: *const T1, a2: *const T1, a3: *const T1) -> *mut T1 {
    let __off = a1.offset_from(a0.as_ptr()) as usize;
    let mut __idx = __off;
    let mut __a1 = a1;
    let mut __a2 = a2;
    while __a2 != a3 {
        a0.insert(__idx, (*__a2).clone());
        __a2 = __a2.add(1);
        __idx += 1;
    }
    a0.as_mut_ptr().add(__off)
}

unsafe fn f54<T1: Default + Clone>(a0: &mut Vec<T1>, a1: usize, a2: T1) {
    let __a0 = a1 as usize;
    a0.resize(__a0, a2)
}

unsafe fn f55<T1: Clone>(a0: &mut Vec<T1>, a1: &mut Vec<T1>) {
    *a0 = std::mem::take(&mut *a1)
}

unsafe fn f56<T1>(a0: &mut Vec<Vec<T1>>) -> *mut Vec<T1> {
    ((a0).last_mut().unwrap())
}

unsafe fn f57<T1>(a0: Vec<T1>) -> *const T1 {
    a0.as_ptr().add(a0.len())
}

unsafe fn f58<T1: Clone>(a0: &mut Vec<T1>, a1: Vec<T1>) {
    *a0 = a1.clone()
}

unsafe fn f59<T1>(a0: &mut Vec<T1>) {
    a0.shrink_to_fit()
}
