extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasTwoInts {
    pub first: i32,
    pub second: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut null_pm: MemberPtrUnsafe<HasTwoInts, i32> =
        <MemberPtrUnsafe<HasTwoInts, i32>>::default();
    let mut first_pm: MemberPtrUnsafe<HasTwoInts, i32> =
        <MemberPtrUnsafe<HasTwoInts, i32>>::from_offset(::std::mem::offset_of!(HasTwoInts, first));
    let mut second_pm: MemberPtrUnsafe<HasTwoInts, i32> =
        <MemberPtrUnsafe<HasTwoInts, i32>>::from_offset(::std::mem::offset_of!(HasTwoInts, second));
    assert!(((first_pm) != (null_pm)));
    assert!(((second_pm) != (null_pm)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
