extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn first_2() -> i32 {
    let mut p: anon_0 = <anon_0>::default();
    p.x = 1;
    p.y = 2;
    return ((p.x) + (p.y));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_0 {
    pub x: i32,
    pub y: i32,
}
pub unsafe fn second_3() -> i32 {
    let mut q: anon_1 = <anon_1>::default();
    q.a = 10_i64;
    q.b = 20_i64;
    return (((q.a) + (q.b)) as i32);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_1 {
    pub a: i64,
    pub b: i64,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((unsafe { first_2() }) == (3)) as i32) != 0));
    assert!(((((unsafe { second_3() }) == (30)) as i32) != 0));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
