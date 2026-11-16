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
    let mut pm: MemberPtrUnsafe<HasTwoInts, i32> =
        <MemberPtrUnsafe<HasTwoInts, i32>>::from_offset(::std::mem::offset_of!(HasTwoInts, second));
    let mut bytes: [u8; 8] = [0_u8; 8];
    match ({ (bytes.as_mut_ptr() as *mut ::libc::c_void) }, {
        ((&mut pm as *mut MemberPtrUnsafe<HasTwoInts, i32>) as *const ::libc::c_void)
    }) {
        (mut __tmp_0, mut __tmp_1) => {
            if ::std::mem::size_of::<MemberPtrUnsafe<HasTwoInts, i32>>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    __tmp_1,
                    __tmp_0,
                    ::std::mem::size_of::<MemberPtrUnsafe<HasTwoInts, i32>>(),
                )
            }
            __tmp_0
        }
    };
    pm = <MemberPtrUnsafe<HasTwoInts, i32>>::from_offset(::std::mem::offset_of!(HasTwoInts, first));
    match (
        { ((&mut pm as *mut MemberPtrUnsafe<HasTwoInts, i32>) as *mut ::libc::c_void) },
        { (bytes.as_mut_ptr() as *const ::libc::c_void) },
    ) {
        (mut __tmp_2, mut __tmp_3) => {
            if ::std::mem::size_of::<MemberPtrUnsafe<HasTwoInts, i32>>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    __tmp_3,
                    __tmp_2,
                    ::std::mem::size_of::<MemberPtrUnsafe<HasTwoInts, i32>>(),
                )
            }
            __tmp_2
        }
    };
    assert!(
        ((pm)
            == (<MemberPtrUnsafe<HasTwoInts, i32>>::from_offset(::std::mem::offset_of!(
                HasTwoInts, second
            ))))
    );
    let mut obj: HasTwoInts = HasTwoInts {
        first: 3,
        second: 4,
    };
    assert!(((*(&mut obj as *mut HasTwoInts).member(pm)) == (4)));
    let mut null_pm: MemberPtrUnsafe<HasTwoInts, i32> =
        <MemberPtrUnsafe<HasTwoInts, i32>>::default();
    match ({ (bytes.as_mut_ptr() as *mut ::libc::c_void) }, {
        ((&mut null_pm as *mut MemberPtrUnsafe<HasTwoInts, i32>) as *const ::libc::c_void)
    }) {
        (mut __tmp_4, mut __tmp_5) => {
            if ::std::mem::size_of::<MemberPtrUnsafe<HasTwoInts, i32>>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    __tmp_5,
                    __tmp_4,
                    ::std::mem::size_of::<MemberPtrUnsafe<HasTwoInts, i32>>(),
                )
            }
            __tmp_4
        }
    };
    null_pm =
        <MemberPtrUnsafe<HasTwoInts, i32>>::from_offset(::std::mem::offset_of!(HasTwoInts, first));
    match (
        { ((&mut null_pm as *mut MemberPtrUnsafe<HasTwoInts, i32>) as *mut ::libc::c_void) },
        { (bytes.as_mut_ptr() as *const ::libc::c_void) },
    ) {
        (mut __tmp_6, mut __tmp_7) => {
            if ::std::mem::size_of::<MemberPtrUnsafe<HasTwoInts, i32>>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    __tmp_7,
                    __tmp_6,
                    ::std::mem::size_of::<MemberPtrUnsafe<HasTwoInts, i32>>(),
                )
            }
            __tmp_6
        }
    };
    assert!((null_pm).is_null());
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
