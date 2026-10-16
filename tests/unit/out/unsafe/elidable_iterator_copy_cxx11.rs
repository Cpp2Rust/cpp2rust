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
    {
        let __a1 = 4;
        v.push(__a1)
    };
    {
        let __a1 = 7;
        v.push(__a1)
    };
    let mut it: UnsafeRandomAccessIterator<i32> = {
        let mut __it = UnsafeRandomAccessIterator::Vec(v.as_mut_ptr());
        while __it != UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len()))
            && *__it.iter_deref() != 7
        {
            __it.iter_inc();
        }
        __it
    };
    assert!(it != UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len())));
    assert!(((*it.iter_deref()) == (7)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
