extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn strlen_0(mut s: *const u8) -> u64 {
    let mut begin: *const u8 = s;
    'loop_: while ((*s) != 0) {
        s.prefix_inc();
    }
    return ((((s as usize - begin as usize) / ::std::mem::size_of::<u8>()) as i64) as u64);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let s: [u8; 6] = [
        ('s' as u8),
        ('t' as u8),
        ('r' as u8),
        ('i' as u8),
        ('n' as u8),
        ('g' as u8),
    ];
    return ((unsafe {
        let _s: *const u8 = (&s[(0) as usize] as *const u8);
        strlen_0(_s)
    }) as i32);
}
