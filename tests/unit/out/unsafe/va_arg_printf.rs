extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn logf_impl_0(mut fmt: *const u8, mut ap: VaList) -> i32 {
    return ((ap.arg::<i32>()) + (ap.arg::<i32>()));
}
pub unsafe fn logf_1(mut fmt: *const u8, args: &[VaArg]) -> i32 {
    let mut ap: VaList = VaList::default();
    ap = VaList::new(args);
    let mut result: i32 = (unsafe {
        let _fmt: *const u8 = fmt;
        let _ap: VaList = ap;
        logf_impl_0(_fmt, _ap)
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
        ((unsafe {
            let _fmt: *const u8 = b"hello %d %d\0".as_ptr();
            logf_1(_fmt, &[10.into(), 32.into()])
        }) == (42))
    );
    assert!(
        ((unsafe {
            let _fmt: *const u8 = b"x %d %d\0".as_ptr();
            logf_1(_fmt, &[1.into(), 2.into()])
        }) == (3))
    );
    return 0;
}
