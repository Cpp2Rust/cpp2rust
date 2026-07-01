// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn t1() -> Vec<u8> {
    Vec::new()
}

fn t2() -> Ptr<u8> {
    Ptr::null()
}

fn f1(a0: Vec<u8>, a1: usize, a2: usize) -> Vec<u8> {
    let mut __tmp1 =
        a0[(a1) as usize..::std::cmp::min((a1 + a2) as usize, a0.len().saturating_sub(1))].to_vec();
    __tmp1.push(0);
    __tmp1
}

fn f3(a0: Vec<u8>, a1: Ptr<u8>) -> Vec<u8> {
    let mut r = a0.clone();
    r.pop();
    r.extend(a1.to_c_string_iterator());
    r.push(0);
    r
}

fn f4(a0: Ptr<Vec<u8>>, a1: Ptr<u8>, a2: usize) -> Ptr<Vec<u8>> {
    a0.with_mut(|__v: &mut Vec<u8>| {
        __v.pop();
        __v.extend(a1.map(|c| c.read()).take((a2) as usize));
        __v.push(0);
    });
    a0
}

fn f5(a0: Ptr<u8>) -> Ptr<u8> {
    a0
}

fn f6(a0: Ptr<u8>) -> Ptr<u8> {
    a0
}

fn f7(a0: Ptr<u8>, a1: usize) -> Vec<u8> {
    a0.map(|c| c.read())
        .take(a1 as usize)
        .chain(std::iter::once(0))
        .collect::<Vec<u8>>()
}

fn f10(a0: Ptr<u8>) -> Vec<u8> {
    a0.to_c_string_iterator()
        .chain(std::iter::once(0))
        .collect::<Vec<u8>>()
}

fn f11(a0: Ptr<u8>) -> Ptr<u8> {
    a0
}

fn f12(a0: Ptr<u8>) -> Ptr<u8> {
    a0
}

fn f14(a0: Ptr<Vec<u8>>, a1: usize, a2: usize, a3: Ptr<u8>, a4: usize) -> Ptr<Vec<u8>> {
    let pos = a1 as usize;
    let end = std::cmp::min(
        pos + a2 as usize,
        (*a0.upgrade().deref()).len().saturating_sub(1),
    );
    a0.with_mut(|__v: &mut Vec<u8>| {
        __v.splice(pos..end, a3.map(|c| c.read()).take((a4) as usize));
    });
    a0
}

fn f15(a0: Ptr<u8>) -> Ptr<u8> {
    a0.to_last()
}

fn f16(a0: Vec<u8>, a1: Ptr<u8>) -> usize {
    let __lookup: Vec<u8> = a1.to_c_string_iterator().collect();
    a0.iter()
        .take(a0.len().saturating_sub(1))
        .rposition(|&x| __lookup.contains(&x))
        .unwrap_or(usize::MAX)
}

// TODO: This should modify a0 in place
fn f17(a0: Vec<u8>, a1: Ptr<u8>) -> Vec<u8> {
    let mut __tmp2 = a0.clone();
    __tmp2.pop();
    __tmp2.extend(a1.to_c_string_iterator());
    __tmp2.push(0);
    __tmp2
}

fn f18(a0: Vec<u8>, a1: Ptr<u8>) -> bool {
    a0.iter()
        .copied()
        .take(a0.len().saturating_sub(1))
        .eq(a1.to_c_string_iterator())
}

fn f20(a0: Ptr<u8>, a1: usize) -> Ptr<u8> {
    a0.offset(a1 as isize)
}

// TODO: This should return a0
fn f21(a0: &mut Vec<u8>, a1: usize, a2: u8) -> Vec<u8> {
    a0.pop();
    a0.resize(a0.len() + (a1) as usize, a2);
    a0.push(0);
    a0.clone()
}

fn f26(a0: Ptr<Vec<u8>>, a1: usize) -> Ptr<u8> {
    if a1 as usize >= (*a0.upgrade().deref()).len().saturating_sub(1) {
        panic!("out of bounds access")
    } else {
        (a0.to_strong().as_pointer() as Ptr<u8>).offset(a1 as isize)
    }
}

fn f2(a0: Vec<u8>) -> usize {
    (a0.len() - 1)
}

fn f8(a0: std::fs::File, a1: std::fs::File) -> Vec<u8> {
    use std::io::Read;
    let mut __bytes: Vec<u8> = Vec::new();
    let mut __f = &a0;
    __f.read_to_end(&mut __bytes)
        .expect("couldn't read the file");
    __bytes.push(0);
    __bytes
}

fn f9(a0: usize, a1: u8) -> Vec<u8> {
    vec![a1; (a0) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0))
        .collect()
}

fn f13(a0: &mut Vec<u8>, a1: usize) {
    a0.pop();
    a0.resize((a1) as usize, 0);
    a0.push(0)
}

fn f19(a0: Vec<u8>) -> usize {
    (a0.len() - 1)
}

fn f22(a0: Vec<u8>) -> bool {
    a0.len() <= 1
}

fn f23() -> Vec<u8> {
    vec![0]
}

fn f24(a0: &mut Vec<u8>) {
    a0.clear();
    a0.push(0)
}

fn f25(a0: &mut Vec<u8>) {
    a0.shrink_to_fit()
}
