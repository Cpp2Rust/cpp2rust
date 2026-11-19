extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    let mut value: i32 = 5;
    let mut ptr: *mut i32 = (&mut value as *mut i32);
    let mut ptr_ptr: *mut *mut i32 = (&mut ptr as *mut *mut i32);
    let mut converted: *const *const i32 = (ptr_ptr as *const *const i32);
    assert!(((converted) == (ptr_ptr as *const *const i32)));
    assert!(((*converted) == ((ptr).cast_const())));
    value = 7;
    assert!(((*(*converted)) == (7)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p10_null_converts_to_null_HasTwoInts {
    pub first: i32,
    pub second: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p10_null_converts_to_null_HasChar {
    pub character: libc::c_char,
}
pub unsafe fn test_1() {
    let mut pm: isize = -1_isize;
    let mut converted: isize = pm;
    assert!(((converted) == -1_isize));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p10_2_data_member_round_trip_HasTwoInts {
    pub first: i32,
    pub second: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p10_2_data_member_round_trip_HasChar {
    pub character: libc::c_char,
}
pub unsafe fn test_2() {
    let mut converted: isize =
        (::std::mem::offset_of!(p10_2_data_member_round_trip_HasTwoInts, second) as isize);
    let mut back: isize = converted;
    assert!(
        ((back)
            == (::std::mem::offset_of!(p10_2_data_member_round_trip_HasTwoInts, second) as isize))
    );
    let mut obj: p10_2_data_member_round_trip_HasTwoInts =
        p10_2_data_member_round_trip_HasTwoInts {
            first: 3,
            second: 4,
        };
    assert!(
        ((*(&mut obj as *mut p10_2_data_member_round_trip_HasTwoInts)
            .wrapping_byte_offset(back)
            .cast::<i32>())
            == (4))
    );
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
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
