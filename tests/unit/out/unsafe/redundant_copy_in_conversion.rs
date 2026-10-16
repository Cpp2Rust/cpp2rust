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
    (*&mut (*m.get_or_insert_with(0.clone(), || (0, <i32>::default()))).1) = 1;
    let mut end: UnsafeBidirectionalIterator<(i32, i32)> = m.end();
    let mut it0: UnsafeBidirectionalIterator<(i32, i32)> = m.find(&0);
    let mut const_it: UnsafeBidirectionalIterator<(i32, i32)> =
        <UnsafeBidirectionalIterator<(i32, i32)>>::from((it0).clone());
    let mut r: i32 = if const_it == <UnsafeBidirectionalIterator<(i32, i32)>>::from((end).clone()) {
        0
    } else {
        1
    };
    r += (unsafe { sink_0(it0.clone()) });
    r += if end == end { 0 } else { 1 };
    assert!(((r) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
