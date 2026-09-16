extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn my_foo_0(mut p: *mut ::libc::c_void) -> i32 {
    return (*(p as *mut i32));
}
pub unsafe fn foo_1(
    mut fn_: Option<unsafe fn(*mut ::libc::c_void) -> i32>,
    mut pi: *mut i32,
) -> i32 {
    return (unsafe { (fn_).unwrap()((pi as *mut i32 as *mut ::libc::c_void)) });
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut fn_: Option<unsafe fn(*mut ::libc::c_void) -> i32> = None;
    assert!((fn_).is_none());
    assert!(((fn_) != (Some(my_foo_0))));
    fn_ = Some(my_foo_0);
    assert!(!((fn_).is_none()));
    assert!(((fn_) == (Some(my_foo_0))));
    let mut a: i32 = 10;
    assert!(((unsafe { foo_1(fn_, (&mut a as *mut i32),) }) == (a)));
    return 0;
}
