extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn strlen_0(mut s: *const core::ffi::c_char) -> usize {
    let mut begin: *const core::ffi::c_char = s;
    'loop_: while ((*s) != 0) {
        s.prefix_inc();
    }
    return ((((s as usize - begin as usize) / ::std::mem::size_of::<core::ffi::c_char>()) as i64)
        as usize);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let s: [core::ffi::c_char; 7] = [
        ('s' as core::ffi::c_char),
        ('t' as core::ffi::c_char),
        ('r' as core::ffi::c_char),
        ('i' as core::ffi::c_char),
        ('n' as core::ffi::c_char),
        ('g' as core::ffi::c_char),
        ('\0' as core::ffi::c_char),
    ];
    return ((unsafe {
        let _s: *const core::ffi::c_char = (&s[(0) as usize] as *const core::ffi::c_char);
        strlen_0(_s)
    }) as i32);
}
