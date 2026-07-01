extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_mut_0(mut str: *mut libc::c_char) {}
pub unsafe fn foo_const_1(mut str: *const libc::c_char) {}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut immutable_strings: [*const libc::c_char; 3] =
        [c"a".as_ptr(), c"b".as_ptr(), c"c".as_ptr()];
    let mut immutable_string: *const libc::c_char = c"hello".as_ptr();
    let mut mutable_string_arr: [libc::c_char; 9] =
        unsafe { ::std::mem::transmute(*b"papanasi\0") };
    let immutable_string_arr: [libc::c_char; 9] = unsafe { ::std::mem::transmute(*b"papanasi\0") };
    let mut immutable_empty: *const libc::c_char = c"".as_ptr();
    let mut mutable_empty_arr: [libc::c_char; 1] = [0 as libc::c_char; 1];
    let immutable_empty_arr: [libc::c_char; 1] = [0 as libc::c_char; 1];
    (unsafe { foo_mut_0(mutable_string_arr.as_mut_ptr()) });
    (unsafe { foo_const_1(c"world".as_ptr()) });
    (unsafe { foo_const_1(immutable_string) });
    (unsafe { foo_const_1(immutable_string_arr.as_ptr()) });
    (unsafe { foo_const_1(c"".as_ptr()) });
    (unsafe { foo_const_1(immutable_empty) });
    (unsafe { foo_const_1(immutable_empty_arr.as_ptr()) });
    let inited_through_init_list: [libc::c_char; 21] =
        unsafe { ::std::mem::transmute(*b"papanasi cu smantana\0") };
    (unsafe { foo_const_1(inited_through_init_list.as_ptr()) });
    return 0;
}
