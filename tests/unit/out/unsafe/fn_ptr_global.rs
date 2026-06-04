extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn double_it_0(mut x: i32) -> i32 {
    return ((x) * (2));
}
pub unsafe fn triple_it_1(mut x: i32) -> i32 {
    return ((x) * (3));
}
pub static mut g_op_2: Option<unsafe fn(i32) -> i32> = unsafe { None };
pub unsafe fn set_op_3(mut fn_: Option<unsafe fn(i32) -> i32>) {
    g_op_2 = fn_;
}
pub unsafe fn call_op_4(mut x: i32) -> i32 {
    if !(unsafe { (&raw const g_op_2).as_ref().unwrap().is_none() }) {
        return (unsafe {
            let _arg0: i32 = x;
            (g_op_2).unwrap()(_arg0)
        });
    }
    return x;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((unsafe {
            let _x: i32 = 5;
            call_op_4(_x)
        }) == (5))
    );
    (unsafe {
        let _fn: Option<unsafe fn(i32) -> i32> = Some(double_it_0);
        set_op_3(_fn)
    });
    assert!(!(unsafe { (&raw const g_op_2).as_ref().unwrap().is_none() }));
    assert!(((g_op_2) == (Some(double_it_0))));
    assert!(
        ((unsafe {
            let _x: i32 = 5;
            call_op_4(_x)
        }) == (10))
    );
    (unsafe {
        let _fn: Option<unsafe fn(i32) -> i32> = Some(triple_it_1);
        set_op_3(_fn)
    });
    assert!(((g_op_2) == (Some(triple_it_1))));
    assert!(
        ((unsafe {
            let _x: i32 = 5;
            call_op_4(_x)
        }) == (15))
    );
    (unsafe {
        let _fn: Option<unsafe fn(i32) -> i32> = None;
        set_op_3(_fn)
    });
    assert!(unsafe { (&raw const g_op_2).as_ref().unwrap().is_none() });
    assert!(
        ((unsafe {
            let _x: i32 = 5;
            call_op_4(_x)
        }) == (5))
    );
    return 0;
}
