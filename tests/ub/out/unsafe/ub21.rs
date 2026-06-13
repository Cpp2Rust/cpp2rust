extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn strlen_0(mut s: *const core::ffi::c_char) -> usize {
    let mut count: usize = 0_usize;
    'loop_: while ((*s.postfix_inc()) != 0) {
        count.prefix_inc();
    }
    return count;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let s: [core::ffi::c_char; 3] = [
        ('s' as core::ffi::c_char),
        ('t' as core::ffi::c_char),
        ('r' as core::ffi::c_char),
    ];
    return ((unsafe {
        let _s: *const core::ffi::c_char = s.as_ptr();
        strlen_0(_s)
    }) as i32);
}
