extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union HasTwoIntMembers {
    pub first: i32,
    pub second: i32,
}
impl Default for HasTwoIntMembers {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut first_pm: MemberPtrUnsafe<HasTwoIntMembers, i32> =
        <MemberPtrUnsafe<HasTwoIntMembers, i32>>::from_offset(0);
    let mut second_pm: MemberPtrUnsafe<HasTwoIntMembers, i32> =
        <MemberPtrUnsafe<HasTwoIntMembers, i32>>::from_offset(0);
    assert!(((first_pm) == (second_pm)));
    assert!(!((first_pm) != (second_pm)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
