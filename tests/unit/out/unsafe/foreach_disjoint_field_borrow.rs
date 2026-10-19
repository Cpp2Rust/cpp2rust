extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub v: Vec<i32>,
    pub a: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = <S>::default();
    {
        let __a1 = 1;
        s.v.push(__a1)
    };
    let mut __begin1: UnsafeRandomAccessIterator<i32> =
        UnsafeRandomAccessIterator::Vec(s.v.as_mut_ptr());
    let mut __end1: UnsafeRandomAccessIterator<i32> =
        UnsafeRandomAccessIterator::Vec(s.v.as_mut_ptr().add(s.v.len()));
    'loop_: while __begin1 != __end1 {
        let mut e: i32 = (*__begin1.iter_deref());
        s.a.postfix_inc();
        __begin1.iter_inc();
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
