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
pub unsafe fn strlen_1(mut s: *mut u8) -> i32 {
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
    let mut s: [u8; 4] = [('a' as u8), ('b' as u8), ('c' as u8), ('\0' as u8)];
    return ((unsafe {
        let _p: *mut i32 = a.as_mut_ptr();
        deref_0(_p)
    }) + (unsafe {
        let _s: *mut u8 = s.as_mut_ptr();
        strlen_1(_s)
    }));
}
