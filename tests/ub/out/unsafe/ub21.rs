extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn strlen_0(mut s: *const u8) -> u64 {
    let mut count: u64 = 0_u64;
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
    let s: [u8; 3] = [('s' as u8), ('t' as u8), ('r' as u8)];
    return ((unsafe {
        let _s: *const u8 = s.as_ptr();
        strlen_0(_s)
    }) as i32);
}
