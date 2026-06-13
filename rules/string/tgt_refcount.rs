// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn t2() -> Ptr<core::ffi::c_char> {
    Ptr::null()
}

fn f1(a0: Vec<core::ffi::c_char>, a1: usize, a2: usize) -> Vec<core::ffi::c_char> {
    let mut __tmp1 =
        a0[(a1) as usize..::std::cmp::min((a1 + a2) as usize, a0.len().saturating_sub(1))].to_vec();
    __tmp1.push(0);
    __tmp1
}

fn f3(a0: Vec<core::ffi::c_char>, a1: Ptr<core::ffi::c_char>) -> Vec<core::ffi::c_char> {
    let mut r = a0.clone();
    r.pop();
    r.extend(a1.to_c_string_iterator());
    r.push(0);
    r
}

fn f4(
    a0: Ptr<Vec<core::ffi::c_char>>,
    a1: Ptr<core::ffi::c_char>,
    a2: usize,
) -> Ptr<Vec<core::ffi::c_char>> {
    a0.with_mut(|__v: &mut Vec<core::ffi::c_char>| {
        __v.pop();
        __v.extend(a1.map(|c| c.read()).take((a2) as usize));
        __v.push(0);
    });
    a0
}

fn f5(a0: Ptr<core::ffi::c_char>) -> Ptr<core::ffi::c_char> {
    a0
}

fn f6(a0: Ptr<core::ffi::c_char>) -> Ptr<core::ffi::c_char> {
    a0
}

fn f7(a0: Ptr<core::ffi::c_char>, a1: usize) -> Vec<core::ffi::c_char> {
    a0.map(|c| c.read())
        .take(a1 as usize)
        .chain(std::iter::once(0))
        .collect::<Vec<core::ffi::c_char>>()
}

fn f10(a0: Ptr<core::ffi::c_char>) -> Vec<core::ffi::c_char> {
    a0.to_c_string_iterator()
        .chain(std::iter::once(0))
        .collect::<Vec<core::ffi::c_char>>()
}

fn f11(a0: Ptr<core::ffi::c_char>) -> Ptr<core::ffi::c_char> {
    a0
}

fn f12(a0: Ptr<core::ffi::c_char>) -> Ptr<core::ffi::c_char> {
    a0
}

fn f14(
    a0: Ptr<Vec<core::ffi::c_char>>,
    a1: usize,
    a2: usize,
    a3: Ptr<core::ffi::c_char>,
    a4: usize,
) -> Ptr<Vec<core::ffi::c_char>> {
    let pos = a1 as usize;
    let end = std::cmp::min(
        pos + a2 as usize,
        (*a0.upgrade().deref()).len().saturating_sub(1),
    );
    a0.with_mut(|__v: &mut Vec<core::ffi::c_char>| {
        __v.splice(pos..end, a3.map(|c| c.read()).take((a4) as usize));
    });
    a0
}

fn f15(a0: Ptr<core::ffi::c_char>) -> Ptr<core::ffi::c_char> {
    a0.to_last()
}

fn f16(a0: Vec<core::ffi::c_char>, a1: Ptr<core::ffi::c_char>) -> usize {
    let __lookup: Vec<core::ffi::c_char> = a1.to_c_string_iterator().collect();
    a0.iter()
        .take(a0.len().saturating_sub(1))
        .rposition(|&x| __lookup.contains(&x))
        .unwrap_or(usize::MAX)
}

// TODO: This should modify a0 in place
fn f17(a0: Vec<core::ffi::c_char>, a1: Ptr<core::ffi::c_char>) -> Vec<core::ffi::c_char> {
    let mut __tmp2 = a0.clone();
    __tmp2.pop();
    __tmp2.extend(a1.to_c_string_iterator());
    __tmp2.push(0);
    __tmp2
}

fn f18(a0: Vec<core::ffi::c_char>, a1: Ptr<core::ffi::c_char>) -> bool {
    a0.iter()
        .copied()
        .take(a0.len().saturating_sub(1))
        .eq(a1.to_c_string_iterator())
}

fn f20(a0: Ptr<core::ffi::c_char>, a1: usize) -> Ptr<core::ffi::c_char> {
    a0.offset(a1 as isize)
}

// TODO: This should return a0
fn f21(
    a0: &mut Vec<core::ffi::c_char>,
    a1: usize,
    a2: core::ffi::c_char,
) -> Vec<core::ffi::c_char> {
    a0.pop();
    a0.resize(a0.len() + (a1) as usize, a2);
    a0.push(0);
    a0.clone()
}

fn f26(a0: Ptr<Vec<core::ffi::c_char>>, a1: usize) -> Ptr<core::ffi::c_char> {
    if a1 as usize >= (*a0.upgrade().deref()).len().saturating_sub(1) {
        panic!("out of bounds access")
    } else {
        (a0.to_strong().as_pointer() as Ptr<core::ffi::c_char>).offset(a1 as isize)
    }
}
