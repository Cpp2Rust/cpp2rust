extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn logf_impl_0(mut fmt: *const libc::c_char, mut ap: VaList) -> i32 {
    &(fmt);
    return ((ap.arg::<i32>()) + (ap.arg::<i32>()));
}
pub unsafe fn logf_1(mut fmt: *const libc::c_char, __args: &[VaArg]) -> i32 {
    let mut ap: VaList = VaList::default();
    ap = VaList::new(__args);
    let mut result: i32 = (unsafe { logf_impl_0(fmt, ap) });
    return result;
}
pub unsafe fn lenf_2(mut fmt: *const libc::c_char, __args: &[VaArg]) -> i32 {
    let mut ap: VaList = VaList::default();
    ap = VaList::new(__args);
    let mut s: *const libc::c_char = ap.arg::<*const libc::c_char>();
    let mut result: i32 = (libc::strlen(s) as i32);
    return result;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut dummy: *const libc::c_char = (c"dummy".as_ptr().cast_mut()).cast_const();
    assert!(
        ((((unsafe {
            logf_1(
                (c"hello %d %d".as_ptr().cast_mut()).cast_const(),
                &[(10).into(), (libc::strlen(dummy)).into()],
            )
        }) == (15)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            logf_1(
                (c"x %d %d".as_ptr().cast_mut()).cast_const(),
                &[(1).into(), (2).into()],
            )
        }) == (3)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe { lenf_2((c"%s".as_ptr().cast_mut()).cast_const(), &[(dummy).into(),]) }) == (5))
            as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            lenf_2(
                (c"%s".as_ptr().cast_mut()).cast_const(),
                &[(if (((*dummy.offset((0) as isize)) as i32) != 0) {
                    dummy
                } else {
                    (c"".as_ptr().cast_mut() as *const libc::c_char)
                })
                .into()],
            )
        }) == (5)) as i32)
            != 0)
    );
    return 0;
}
