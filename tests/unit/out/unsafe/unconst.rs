extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn set_0(r: *mut i32) {
    (*r) = 7;
}
pub unsafe fn set_through_1(v: *const i32) {
    (unsafe { set_0((v).cast_mut()) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let a: i32 = 1;
    let mut p: *const i32 = (&a as *const i32);
    let mut q: *mut i32 =
        (((((p) as *const ::libc::c_void) as u64) as *mut ::libc::c_void) as *mut i32);
    assert!(((p) == ((q).cast_const())));
    let mut v: i32 = 1;
    let cr: *const i32 = &v;
    (unsafe { set_0((cr).cast_mut()) });
    assert!(((v) == (7)));
    v = 0;
    (unsafe { set_through_1(&v) });
    assert!(((v) == (7)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
