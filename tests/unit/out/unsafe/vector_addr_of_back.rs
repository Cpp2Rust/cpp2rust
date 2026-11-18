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
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut outer: Vec<Vec<i32>> = Vec::new();
    let mut inner: Vec<i32> = Vec::new();
    {
        let a0_clone = inner.clone();
        outer.push(a0_clone)
    };
    let mut sink: *mut Vec<i32> = ({
        let __n = outer.len() - 1;
        &raw mut outer[__n]
    });
    assert!((((*(sink).cast_const()).len()) == (0_usize)));
    let mut p: *mut Vec<Vec<i32>> = (&mut outer as *mut Vec<Vec<i32>>);
    sink = ({
        let __n = (*p).len() - 1;
        &raw mut (&mut (*p))[__n]
    });
    assert!((((*(sink).cast_const()).len()) == (0_usize)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
