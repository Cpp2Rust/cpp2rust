extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn strlen_0(mut s: *const core::ffi::c_char, mut n: i32) -> i32 {
    return if ((*s) != 0) {
        (unsafe {
            let _s: *const core::ffi::c_char = s.offset((1) as isize);
            let _n: i32 = ((n) + (1));
            strlen_0(_s, _n)
        })
    } else {
        n
    };
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let s: [core::ffi::c_char; 4] = [
        ('s' as core::ffi::c_char),
        ('t' as core::ffi::c_char),
        ('r' as core::ffi::c_char),
        ('\0' as core::ffi::c_char),
    ];
    return (unsafe {
        let _s: *const core::ffi::c_char = (&s[(0) as usize] as *const core::ffi::c_char);
        strlen_0(_s, 0)
    });
}
