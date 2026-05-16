// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn types() {
    let t1: Vec<u8> = Vec::new();
    let t2: *mut u8 = ::std::ptr::null_mut();
}

unsafe fn f1(a0: Vec<u8>, a1: usize, a2: usize) -> Vec<u8> {
    let mut __tmp1 = a0[(a1) as usize..::std::cmp::min((a1 + a2) as usize, a0.len() - 1)].to_vec();
    __tmp1.push(0);
    __tmp1
}
unsafe fn f2(a0: Vec<u8>) -> u64 {
    (a0.len() - 1) as u64
}
unsafe fn f3(a0: Vec<u8>, a1: *const u8) -> Vec<u8> {
    let mut __tmp2 = a0.clone();
    __tmp2.pop();
    let __from = a1;
    __tmp2.extend_from_slice(::std::slice::from_raw_parts(
        __from,
        (0..).position(|i| *__from.add(i) == 0).unwrap(),
    ));
    __tmp2.push(0);
    __tmp2
}
unsafe fn f4(a0: &mut Vec<u8>, a1: *mut u8, a2: usize) {
    a0.splice(a0.len().saturating_sub(1)..a0.len(), {
        let mut v = ::std::slice::from_raw_parts(a1, a2 as usize).to_vec();
        v.push(0);
        v
    });
}
unsafe fn f5(a0: Vec<u8>) -> *const u8 {
    a0.as_ptr()
}
unsafe fn f6(a0: &mut Vec<u8>) -> *const u8 {
    a0.as_mut_ptr()
}
unsafe fn f7(a0: *const u8, a1: usize) -> Vec<u8> {
    std::slice::from_raw_parts(a0, a1 as usize)
        .to_vec()
        .iter()
        .copied()
        .chain(std::iter::once(0))
        .collect()
}
// TODO: this does not care for a1
use std::io::Read;

unsafe fn f8(a0: std::fs::File, a1: std::fs::File) -> Vec<u8> {
    let mut __buf: Vec<u8> = Vec::new();
    let mut __f = &a0;
    __f.read_to_end(&mut __buf).expect("couldn't read the file");
    __buf.push(0);
    __buf
}

unsafe fn f9(a0: usize, a1: u8) -> Vec<u8> {
    vec![a1; (a0) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0))
        .collect()
}
unsafe fn f10(a0: *const u8) -> Vec<u8> {
    let s = a0;
    std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
}
unsafe fn f11(a0: Vec<u8>) -> *const u8 {
    a0.as_ptr()
}
unsafe fn f12(a0: &mut Vec<u8>) -> *mut u8 {
    a0.as_mut_ptr()
}
unsafe fn f13(a0: &mut Vec<u8>, a1: usize) {
    a0.pop();
    a0.resize((a1) as usize, 0);
    a0.push(0)
}
unsafe fn f14(a0: &mut Vec<u8>, a1: usize, a2: usize, a3: *const u8, a4: usize) {
    a0.splice(
        a1 as usize..a1 as usize + a2 as usize,
        ::std::slice::from_raw_parts(a3, a4 as usize).to_vec(),
    );
}
unsafe fn f15(a0: &mut Vec<u8>) -> *mut u8 {
    a0.as_mut_ptr().add(a0.len() - 1)
}
unsafe fn f16(a0: Vec<u8>, a1: *const u8) -> u64 {
    match a0.iter().rposition(|&c| {
        ::std::ffi::CStr::from_ptr(a1 as *const i8)
            .to_str()
            .unwrap()
            .contains(c as u8 as char)
    }) {
        Some(idx) => idx as u64,
        None => u64::MAX,
    }
}
unsafe fn f17(a0: Vec<u8>, a1: *const u8) -> Vec<u8> {
    let mut __tmp2 = a0.clone();
    __tmp2.pop();
    let __from = a1;
    __tmp2.extend_from_slice(::std::slice::from_raw_parts(
        __from,
        (0..).position(|i| *__from.add(i) == 0).unwrap(),
    ));
    __tmp2.push(0);
    __tmp2
}
unsafe fn f18(a0: Vec<u8>, a1: *const u8) -> bool {
    a0 == {
        let s = a1;
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    }
}
unsafe fn f19(a0: Vec<u8>) -> u64 {
    (a0.len() - 1) as u64
}
unsafe fn f20(a0: *mut u8, a1: usize) -> *mut u8 {
    a0.add(a1 as usize)
}
unsafe fn f21(a0: &mut Vec<u8>, a1: usize, a2: u8) {
    a0.splice(
        a0.len() - 1..a0.len() - 1,
        ::std::vec::from_elem(a2, a1 as usize),
    );
}

unsafe fn f22(a0: Vec<u8>) -> bool {
    a0.len() <= 1
}

unsafe fn f23() -> Vec<u8> {
    vec![0]
}

unsafe fn f24(a0: &mut Vec<u8>) {
    a0.clear();
    a0.push(0)
}

unsafe fn f25(a0: &mut Vec<u8>) {
    a0.shrink_to_fit()
}

unsafe fn f26(a0: &mut Vec<u8>, a1: usize) -> *mut u8 {
    if a1 as usize >= a0.len() - 1 {
        panic!("out of bounds access")
    } else {
        &mut a0[a1 as usize]
    }
}
