extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn square_0(mut x: i32) -> i32 {
    return ((x) * (x));
}
pub unsafe fn negate_1(mut x: i32) -> i32 {
    return -x;
}
pub unsafe fn add_2(mut a: i32, mut b: i32) -> i32 {
    return ((a) + (b));
}
pub unsafe fn apply_unary_3(mut x: i32, __args: &[VaArg]) -> i32 {
    let mut ap: VaList = VaList::default();
    ap = VaList::new(__args);
    let mut fn_: Option<unsafe fn(i32) -> i32> = std::mem::transmute::<
        *mut ::libc::c_void,
        Option<unsafe fn(i32) -> i32>,
    >(ap.arg::<*mut ::libc::c_void>());
    let mut result: i32 = (unsafe {
        let _arg0: i32 = x;
        (fn_).unwrap()(_arg0)
    });
    return result;
}
pub unsafe fn apply_binary_4(mut a: i32, mut b: i32, __args: &[VaArg]) -> i32 {
    let mut ap: VaList = VaList::default();
    ap = VaList::new(__args);
    let mut fn_: Option<unsafe fn(i32, i32) -> i32> = std::mem::transmute::<
        *mut ::libc::c_void,
        Option<unsafe fn(i32, i32) -> i32>,
    >(ap.arg::<*mut ::libc::c_void>());
    let mut result: i32 = (unsafe {
        let _arg0: i32 = a;
        let _arg1: i32 = b;
        (fn_).unwrap()(_arg0, _arg1)
    });
    return result;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((((unsafe {
            let _x: i32 = 5;
            apply_unary_3(
                _x,
                &[
                    (Some(square_0).map_or(::std::ptr::null_mut(), |f| f as *mut ::libc::c_void))
                        .into(),
                ],
            )
        }) == (25)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _x: i32 = 7;
            apply_unary_3(
                _x,
                &[
                    (Some(negate_1).map_or(::std::ptr::null_mut(), |f| f as *mut ::libc::c_void))
                        .into(),
                ],
            )
        }) == (-7_i32)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _a: i32 = 3;
            let _b: i32 = 4;
            apply_binary_4(
                _a,
                _b,
                &[
                    (Some(add_2).map_or(::std::ptr::null_mut(), |f| f as *mut ::libc::c_void))
                        .into(),
                ],
            )
        }) == (7)) as i32)
            != 0)
    );
    return 0;
}
