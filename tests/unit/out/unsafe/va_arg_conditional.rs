extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn conditional_log_0(mut verbose: i32, mut fmt: *const u8, __args: &[VaArg]) -> i32 {
    if (verbose != 0) {
        let mut ap: VaList = VaList::default();
        ap = VaList::new(__args);
        let mut result: i32 = ap.arg::<i32>();
        return result;
    }
    return -1_i32;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((((unsafe {
            let _verbose: i32 = 1;
            let _fmt: *const u8 = b"%d\0".as_ptr().cast_mut().cast_const();
            conditional_log_0(_verbose, _fmt, &[42.into()])
        }) == (42)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _verbose: i32 = 0;
            let _fmt: *const u8 = b"%d\0".as_ptr().cast_mut().cast_const();
            conditional_log_0(_verbose, _fmt, &[99.into()])
        }) == (-1_i32)) as i32)
            != 0)
    );
    return 0;
}
