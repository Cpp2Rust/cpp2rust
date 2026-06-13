extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn deref_0(mut p: *mut i32) -> i32 {
    return (*p);
}
pub unsafe fn strlen_1(mut s: *mut ::core::ffi::c_char) -> i32 {
    let mut c: i32 = 0;
    'loop_: while ((*s.postfix_inc()) != 0) {
        c.prefix_inc();
    }
    return c;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: [i32; 2] = [1, 2];
    let mut s: [::core::ffi::c_char; 4] = [
        ('a' as ::core::ffi::c_char),
        ('b' as ::core::ffi::c_char),
        ('c' as ::core::ffi::c_char),
        ('\0' as ::core::ffi::c_char),
    ];
    return ((unsafe {
        let _p: *mut i32 = a.as_mut_ptr();
        deref_0(_p)
    }) + (unsafe {
        let _s: *mut ::core::ffi::c_char = s.as_mut_ptr();
        strlen_1(_s)
    }));
}
