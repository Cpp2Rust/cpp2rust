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
pub struct p4_1_both_null_HasTwoInts {
    pub first: i32,
    pub second: i32,
}
pub unsafe fn test_0() {
    let mut first_null: isize = -1_isize;
    let mut second_null: isize = -1_isize;
    assert!(((first_null) == (second_null)));
    assert!(!((first_null) != (second_null)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p4_2_one_null_HasTwoInts {
    pub first: i32,
    pub second: i32,
}
pub unsafe fn test_1() {
    let mut null_pm: isize = -1_isize;
    let mut first_pm: isize = (::std::mem::offset_of!(p4_2_one_null_HasTwoInts, first) as isize);
    assert!(((first_pm) != (null_pm)));
    assert!(!((first_pm) == (null_pm)));
    assert!(((null_pm) != (first_pm)));
    assert!(!((null_pm) == (first_pm)));
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union p4_5_members_of_same_union_HasTwoIntMembers {
    pub first: i32,
    pub second: i32,
}
impl Default for p4_5_members_of_same_union_HasTwoIntMembers {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub unsafe fn test_2() {
    let mut first_pm: isize = 0_isize;
    let mut second_pm: isize = 0_isize;
    assert!(((first_pm) == (second_pm)));
    assert!(!((first_pm) != (second_pm)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p4_6_data_members_HasTwoInts {
    pub first: i32,
    pub second: i32,
}
pub unsafe fn test_3() {
    let mut first_pm: isize =
        (::std::mem::offset_of!(p4_6_data_members_HasTwoInts, first) as isize);
    let mut also_first_pm: isize =
        (::std::mem::offset_of!(p4_6_data_members_HasTwoInts, first) as isize);
    let mut second_pm: isize =
        (::std::mem::offset_of!(p4_6_data_members_HasTwoInts, second) as isize);
    assert!(((first_pm) == (also_first_pm)));
    assert!(!((first_pm) != (also_first_pm)));
    assert!(((first_pm) != (second_pm)));
    assert!(!((first_pm) == (second_pm)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    (unsafe { test_1() });
    (unsafe { test_2() });
    (unsafe { test_3() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
