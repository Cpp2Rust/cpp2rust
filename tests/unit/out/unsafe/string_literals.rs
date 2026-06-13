extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_mut_0(mut str: *mut core::ffi::c_char) {}
pub unsafe fn foo_const_1(mut str: *const core::ffi::c_char) {}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut immutable_strings: [*const core::ffi::c_char; 3] =
        [c"a".as_ptr(), c"b".as_ptr(), c"c".as_ptr()];
    let mut immutable_string: *const core::ffi::c_char = c"hello".as_ptr();
    let mut mutable_string_arr: [core::ffi::c_char; 9] = libcc2rs::char_array(b"papanasi\0");
    let immutable_string_arr: [core::ffi::c_char; 9] = libcc2rs::char_array(b"papanasi\0");
    let mut immutable_empty: *const core::ffi::c_char = c"".as_ptr();
    let mut mutable_empty_arr: [core::ffi::c_char; 1] = [0 as core::ffi::c_char; 1];
    let immutable_empty_arr: [core::ffi::c_char; 1] = [0 as core::ffi::c_char; 1];
    (unsafe {
        let _str: *mut core::ffi::c_char = mutable_string_arr.as_mut_ptr();
        foo_mut_0(_str)
    });
    (unsafe { foo_const_1(c"world".as_ptr()) });
    (unsafe {
        let _str: *const core::ffi::c_char = immutable_string;
        foo_const_1(_str)
    });
    (unsafe {
        let _str: *const core::ffi::c_char = immutable_string_arr.as_ptr();
        foo_const_1(_str)
    });
    (unsafe { foo_const_1(c"".as_ptr()) });
    (unsafe {
        let _str: *const core::ffi::c_char = immutable_empty;
        foo_const_1(_str)
    });
    (unsafe {
        let _str: *const core::ffi::c_char = immutable_empty_arr.as_ptr();
        foo_const_1(_str)
    });
    let inited_through_init_list: [core::ffi::c_char; 21] =
        libcc2rs::char_array(b"papanasi cu smantana\0");
    (unsafe {
        let _str: *const core::ffi::c_char = inited_through_init_list.as_ptr();
        foo_const_1(_str)
    });
    return 0;
}
