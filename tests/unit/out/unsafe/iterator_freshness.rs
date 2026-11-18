extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(mut a0: UnsafeRandomAccessIterator<i32>) {}
pub unsafe fn deref_1(mut a0: UnsafeRandomAccessIterator<i32>) -> i32 {
    return (*a0.iter_deref());
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut vec_: Vec<i32> = (0..(4_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>();
    let mut it: UnsafeRandomAccessIterator<i32> = vec_.begin();
    (unsafe { foo_0((it).clone()) });
    vec_[(0_usize)] = 7;
    assert!(((unsafe { deref_1((it).clone(),) }) == (7)));
    assert!(((*it.iter_deref()) == (7)));
    let mut moved: UnsafeRandomAccessIterator<i32> = (it).clone();
    assert!(((*moved.iter_deref()) == (7)));
    assert!(((*it.iter_deref()) == (7)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
