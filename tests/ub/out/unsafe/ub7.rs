extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn strlen_0(mut s: *const libc::c_char) -> usize {
    let mut begin: *const libc::c_char = s;
    'loop_: while ((*s) != 0) {
        s.prefix_inc();
    }
    return ((((s as usize - begin as usize) / ::std::mem::size_of::<libc::c_char>()) as i64)
        as usize);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let s: [libc::c_char; 6] = [
        ('s' as libc::c_char),
        ('t' as libc::c_char),
        ('r' as libc::c_char),
        ('i' as libc::c_char),
        ('n' as libc::c_char),
        ('g' as libc::c_char),
    ];
    return ((unsafe { strlen_0((&s[(0) as usize] as *const libc::c_char)) }) as i32);
}
