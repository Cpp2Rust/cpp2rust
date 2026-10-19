extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<i32> = Vec::new();
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        {
            let a0_clone = i.clone();
            v.push(a0_clone)
        };
        i.prefix_inc();
    }
    let mut sum: i32 = 0;
    'loop_: for __begin1 in UnsafeIterRange::new(
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr()),
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len())),
    ) {
        let mut x: i32 = (*__begin1.iter_deref());
        sum += x;
    }
    assert!(((sum) == (45)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
