extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sink_0(mut it: UnsafeBidirectionalIterator<(i32, i32)>) -> i32 {
    let mut cit: UnsafeBidirectionalIterator<(i32, i32)> =
        <UnsafeBidirectionalIterator<(i32, i32)>>::from((it).clone());
    return if cit == <UnsafeBidirectionalIterator<(i32, i32)>>::from((it).clone()) {
        (*it.iter_deref()).1
    } else {
        0
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut m: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
    (*m.subscript_move(&mut 0, |__k| cpp_move_unsafe!(__k))) = 1;
    let mut end: UnsafeBidirectionalIterator<(i32, i32)> = m.end();
    let mut it0: UnsafeBidirectionalIterator<(i32, i32)> = m.find(&0);
    let mut const_it: UnsafeBidirectionalIterator<(i32, i32)> =
        <UnsafeBidirectionalIterator<(i32, i32)>>::from((it0).clone());
    let mut r: i32 = if const_it == <UnsafeBidirectionalIterator<(i32, i32)>>::from((end).clone()) {
        0
    } else {
        1
    };
    r += (unsafe { sink_0((it0).clone()) });
    r += if end == end { 0 } else { 1 };
    assert!(((r) == (2)));
    let mut values: Vec<u32> = Vec::from([7_u32, 8_u32, 9_u32]);
    let mut middle: UnsafeRandomAccessIterator<u32> = values.begin().iter_add(1_isize);
    let mut const_middle: UnsafeRandomAccessIterator<u32> =
        <UnsafeRandomAccessIterator<u32>>::from((middle).clone());
    assert!(((*const_middle.iter_deref()) == (8_u32)));
    assert!(const_middle == middle);
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
