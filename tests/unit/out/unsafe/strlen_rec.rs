extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn strlen_0(mut s: *const u8, mut n: i32) -> i32 {
    return if ((*s) != 0) {
        (unsafe {
            let _s: *const u8 = s.offset((1) as isize);
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
    let s: [u8; 4] = [('s' as u8), ('t' as u8), ('r' as u8), ('\0' as u8)];
    return (unsafe {
        let _s: *const u8 = (&s[(0) as usize] as *const u8);
        let _n: i32 = 0;
        strlen_0(_s, _n)
    });
}
