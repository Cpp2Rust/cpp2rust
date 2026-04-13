extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn double_it_0(mut x: i32) -> i32 {
    return ((x) * (2));
}
pub unsafe fn test_roundtrip_1() {
    let mut fn_: Option<unsafe fn(i32) -> i32> = Some(double_it_0 as _);
    assert!(
        ((unsafe {
            let _arg0: i32 = 5;
            (fn_).unwrap()(_arg0)
        }) == (10))
    );
    let mut gfn: Option<unsafe fn()> = (fn_ as Option<unsafe fn()>);
    assert!(!((gfn).is_none()));
    let mut fn2: Option<unsafe fn(i32) -> i32> = (gfn as Option<unsafe fn(i32) -> i32>);
    assert!(
        ((unsafe {
            let _arg0: i32 = 5;
            (fn2).unwrap()(_arg0)
        }) == (10))
    );
    assert!(((fn2) == (fn_)));
}
pub unsafe fn test_double_cast_2() {
    let mut fn_: Option<unsafe fn(i32) -> i32> = Some(double_it_0 as _);
    let mut fn2: Option<unsafe fn(i32) -> i32> =
        ((fn_ as Option<unsafe fn()>) as Option<unsafe fn(i32) -> i32>);
    assert!(
        ((unsafe {
            let _arg0: i32 = 5;
            (fn2).unwrap()(_arg0)
        }) == (10))
    );
    assert!(((fn2) == (fn_)));
}
#[derive(Copy, Clone, Default)]
pub struct Command {
    pub data: *mut ::libc::c_void,
}
pub unsafe fn test_void_ptr_to_fn_3() {
    let mut cmd: Command = <Command>::default();
    cmd.data = (Some(double_it_0 as _) as *mut ::libc::c_void);
    let mut fn_: Option<unsafe fn(i32) -> i32> = (cmd.data as Option<unsafe fn(i32) -> i32>);
    assert!(
        ((unsafe {
            let _arg0: i32 = 5;
            (fn_).unwrap()(_arg0)
        }) == (10))
    );
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_roundtrip_1() });
    (unsafe { test_double_cast_2() });
    (unsafe { test_void_ptr_to_fn_3() });
    return 0;
}
