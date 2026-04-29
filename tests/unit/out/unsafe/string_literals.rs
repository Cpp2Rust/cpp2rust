extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_mut_0(mut str: *mut u8) {}
pub unsafe fn foo_const_1(mut str: *const u8) {}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut mutable_strings: [*mut u8; 3] = [
        b"a\0".as_ptr().cast_mut(),
        b"b\0".as_ptr().cast_mut(),
        b"c\0".as_ptr().cast_mut(),
    ];
    let mut immutable_strings: [*const u8; 3] = [b"a\0".as_ptr(), b"b\0".as_ptr(), b"c\0".as_ptr()];
    let mut mutable_string: *mut u8 = b"hello\0".as_ptr().cast_mut();
    let mut immutable_string: *const u8 = b"hello\0".as_ptr();
    let mut mutable_string_arr: [u8; 9] = *b"papanasi\0";
    let immutable_string_arr: [u8; 9] = *b"papanasi\0";
    let mut mutable_empty: *mut u8 = b"\0".as_ptr().cast_mut();
    let mut immutable_empty: *const u8 = b"\0".as_ptr();
    let mut mutable_empty_arr: [u8; 1] = *b"\0";
    let immutable_empty_arr: [u8; 1] = *b"\0";
    (unsafe {
        let _str: *mut u8 = b"world\0".as_ptr().cast_mut();
        foo_mut_0(_str)
    });
    (unsafe {
        let _str: *mut u8 = mutable_string;
        foo_mut_0(_str)
    });
    (unsafe {
        let _str: *mut u8 = mutable_string_arr.as_mut_ptr();
        foo_mut_0(_str)
    });
    (unsafe {
        let _str: *const u8 = b"world\0".as_ptr();
        foo_const_1(_str)
    });
    (unsafe {
        let _str: *const u8 = mutable_string.cast_const();
        foo_const_1(_str)
    });
    (unsafe {
        let _str: *const u8 = immutable_string;
        foo_const_1(_str)
    });
    (unsafe {
        let _str: *const u8 = mutable_string_arr.as_mut_ptr().cast_const();
        foo_const_1(_str)
    });
    (unsafe {
        let _str: *const u8 = immutable_string_arr.as_ptr();
        foo_const_1(_str)
    });
    (unsafe {
        let _str: *const u8 = b"\0".as_ptr();
        foo_const_1(_str)
    });
    (unsafe {
        let _str: *const u8 = mutable_empty.cast_const();
        foo_const_1(_str)
    });
    (unsafe {
        let _str: *const u8 = immutable_empty;
        foo_const_1(_str)
    });
    (unsafe {
        let _str: *const u8 = mutable_empty_arr.as_mut_ptr().cast_const();
        foo_const_1(_str)
    });
    (unsafe {
        let _str: *const u8 = immutable_empty_arr.as_ptr();
        foo_const_1(_str)
    });
    return 0;
}
