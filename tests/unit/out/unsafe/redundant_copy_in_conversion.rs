extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sink_0(mut it: UnsafeMapIterator<i32, i32>) -> i32 {
    let mut cit: UnsafeMapIterator<i32, i32> = it.clone();
    return if cit == it.clone() { *it.second() } else { 0 };
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut m: BTreeMap<i32, Box<i32>> = BTreeMap::new();
    (*m.entry(0).or_default().as_mut()) = 1;
    let mut end: UnsafeMapIterator<i32, i32> =
        UnsafeMapIterator::end(&m as *const BTreeMap<i32, Box<i32>>);
    let mut it0: UnsafeMapIterator<i32, i32> =
        UnsafeMapIterator::find_key(&m as *const BTreeMap<i32, Box<i32>>, &0);
    let mut const_it: UnsafeMapIterator<i32, i32> = it0.clone();
    let mut r: i32 = if const_it == end.clone() { 0 } else { 1 };
    r += (unsafe {
        let _it: UnsafeMapIterator<i32, i32> = it0.clone();
        sink_0(_it)
    });
    r += if end == end { 0 } else { 1 };
    return r;
}
