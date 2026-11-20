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
pub struct p2_pointer_to_data_member_bytes_round_trip_HasTwoInts {
    pub first: i32,
    pub second: i32,
}
pub unsafe fn test_0() {
    let mut pm: isize = (::std::mem::offset_of!(
        p2_pointer_to_data_member_bytes_round_trip_HasTwoInts,
        second
    ) as isize);
    let mut bytes: [u8; 8] = [0_u8; 8];
    {
        let mut __tmp_0 = { (bytes.as_mut_ptr() as *mut ::libc::c_void) };
        let mut __tmp_1 = { ((&mut pm as *mut isize) as *const ::libc::c_void) };
        if ::std::mem::size_of::<isize>() != 0 {
            ::std::ptr::copy_nonoverlapping(__tmp_1, __tmp_0, ::std::mem::size_of::<isize>())
        }
        __tmp_0
    };
    pm = (::std::mem::offset_of!(p2_pointer_to_data_member_bytes_round_trip_HasTwoInts, first)
        as isize);
    {
        let mut __tmp_2 = { ((&mut pm as *mut isize) as *mut ::libc::c_void) };
        let mut __tmp_3 = { (bytes.as_mut_ptr() as *const ::libc::c_void) };
        if ::std::mem::size_of::<isize>() != 0 {
            ::std::ptr::copy_nonoverlapping(__tmp_3, __tmp_2, ::std::mem::size_of::<isize>())
        }
        __tmp_2
    };
    assert!(
        ((pm)
            == (::std::mem::offset_of!(
                p2_pointer_to_data_member_bytes_round_trip_HasTwoInts,
                second
            ) as isize))
    );
    let mut obj: p2_pointer_to_data_member_bytes_round_trip_HasTwoInts =
        p2_pointer_to_data_member_bytes_round_trip_HasTwoInts {
            first: 3,
            second: 4,
        };
    assert!(
        ((*(&mut obj as *mut p2_pointer_to_data_member_bytes_round_trip_HasTwoInts)
            .wrapping_byte_offset(pm)
            .cast::<i32>())
            == (4))
    );
    let mut null_pm: isize = -1_isize;
    {
        let mut __tmp_4 = { (bytes.as_mut_ptr() as *mut ::libc::c_void) };
        let mut __tmp_5 = { ((&mut null_pm as *mut isize) as *const ::libc::c_void) };
        if ::std::mem::size_of::<isize>() != 0 {
            ::std::ptr::copy_nonoverlapping(__tmp_5, __tmp_4, ::std::mem::size_of::<isize>())
        }
        __tmp_4
    };
    null_pm = (::std::mem::offset_of!(p2_pointer_to_data_member_bytes_round_trip_HasTwoInts, first)
        as isize);
    {
        let mut __tmp_6 = { ((&mut null_pm as *mut isize) as *mut ::libc::c_void) };
        let mut __tmp_7 = { (bytes.as_mut_ptr() as *const ::libc::c_void) };
        if ::std::mem::size_of::<isize>() != 0 {
            ::std::ptr::copy_nonoverlapping(__tmp_7, __tmp_6, ::std::mem::size_of::<isize>())
        }
        __tmp_6
    };
    assert!(((null_pm) == -1_isize));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p3_pointer_to_data_member_bytes_copied_to_other_object_HasTwoInts {
    pub first: i32,
    pub second: i32,
}
pub unsafe fn test_1() {
    let mut source: isize = (::std::mem::offset_of!(
        p3_pointer_to_data_member_bytes_copied_to_other_object_HasTwoInts,
        second
    ) as isize);
    let mut destination: isize = (::std::mem::offset_of!(
        p3_pointer_to_data_member_bytes_copied_to_other_object_HasTwoInts,
        first
    ) as isize);
    {
        let mut __tmp_8 = { ((&mut destination as *mut isize) as *mut ::libc::c_void) };
        let mut __tmp_9 = { ((&mut source as *mut isize) as *const ::libc::c_void) };
        if ::std::mem::size_of::<isize>() != 0 {
            ::std::ptr::copy_nonoverlapping(__tmp_9, __tmp_8, ::std::mem::size_of::<isize>())
        }
        __tmp_8
    };
    assert!(((destination) == (source)));
    let mut obj: p3_pointer_to_data_member_bytes_copied_to_other_object_HasTwoInts =
        p3_pointer_to_data_member_bytes_copied_to_other_object_HasTwoInts {
            first: 3,
            second: 4,
        };
    assert!(
        ((*(&mut obj as *mut p3_pointer_to_data_member_bytes_copied_to_other_object_HasTwoInts)
            .wrapping_byte_offset(destination)
            .cast::<i32>())
            == (4))
    );
    let mut null_source: isize = -1_isize;
    {
        let mut __tmp_10 = { ((&mut destination as *mut isize) as *mut ::libc::c_void) };
        let mut __tmp_11 = { ((&mut null_source as *mut isize) as *const ::libc::c_void) };
        if ::std::mem::size_of::<isize>() != 0 {
            ::std::ptr::copy_nonoverlapping(__tmp_11, __tmp_10, ::std::mem::size_of::<isize>())
        }
        __tmp_10
    };
    assert!(((destination) == -1_isize));
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
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
