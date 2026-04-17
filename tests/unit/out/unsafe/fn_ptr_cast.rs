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
    let mut fn_: Option<unsafe fn(i32) -> i32> = Some(double_it_0);
    assert!(
        ((unsafe {
            let _arg0: i32 = 5;
            (fn_).unwrap()(_arg0)
        }) == (10))
    );
    let mut gfn: Option<unsafe fn()> =
        std::mem::transmute::<Option<unsafe fn(i32) -> i32>, Option<unsafe fn()>>(fn_);
    assert!(!((gfn).is_none()));
    let mut fn2: Option<unsafe fn(i32) -> i32> =
        std::mem::transmute::<Option<unsafe fn()>, Option<unsafe fn(i32) -> i32>>(gfn);
    assert!(
        ((unsafe {
            let _arg0: i32 = 5;
            (fn2).unwrap()(_arg0)
        }) == (10))
    );
    assert!(((fn2) == (fn_)));
}
pub unsafe fn test_double_cast_2() {
    let mut fn_: Option<unsafe fn(i32) -> i32> = Some(double_it_0);
    let mut fn2: Option<unsafe fn(i32) -> i32> =
        std::mem::transmute::<Option<unsafe fn()>, Option<unsafe fn(i32) -> i32>>(
            std::mem::transmute::<Option<unsafe fn(i32) -> i32>, Option<unsafe fn()>>(fn_),
        );
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
    cmd.data = std::mem::transmute::<Option<unsafe fn(i32) -> i32>, *mut ::libc::c_void>(Some(
        double_it_0,
    ));
    let mut fn_: Option<unsafe fn(i32) -> i32> =
        std::mem::transmute::<*mut ::libc::c_void, Option<unsafe fn(i32) -> i32>>(cmd.data);
    assert!(
        ((unsafe {
            let _arg0: i32 = 5;
            (fn_).unwrap()(_arg0)
        }) == (10))
    );
}
pub unsafe fn add_offset_4(mut base: *mut i32, mut offset: i32) -> i32 {
    return ((*base) + (offset));
}
pub unsafe fn test_call_through_cast_5() {
    let mut gfn: Option<unsafe fn(*mut ::libc::c_void, i32) -> i32> = std::mem::transmute::<
        Option<unsafe fn(*mut i32, i32) -> i32>,
        Option<unsafe fn(*mut ::libc::c_void, i32) -> i32>,
    >(Some(add_offset_4));
    let mut val: i32 = 100;
    let mut result: i32 = (unsafe {
        let _arg0: *mut ::libc::c_void =
            ((&mut val as *mut i32) as *mut i32 as *mut ::libc::c_void);
        let _arg1: i32 = 42;
        (gfn).unwrap()(_arg0, _arg1)
    });
    assert!(((result) == (142)));
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
    (unsafe { test_call_through_cast_5() });
    return 0;
}
