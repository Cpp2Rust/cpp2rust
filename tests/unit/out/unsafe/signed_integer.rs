extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x1: i32 = -1_i32;
    assert!(((((x1) == (-1_i32)) as i32) != 0));
    let mut x2: i8 = -1_i8;
    assert!(((((x2 as i32) == (-1_i32)) as i32) != 0));
    let mut u1: u32 = 5_u32;
    let mut u2: u32 = (u1).wrapping_neg();
    assert!(((((u2) == (4294967291_u32)) as i32) != 0));
    let mut c1: libc::c_char = (-1 as libc::c_char);
    assert!((((((c1 as u8) as i32) == (255)) as i32) != 0));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
