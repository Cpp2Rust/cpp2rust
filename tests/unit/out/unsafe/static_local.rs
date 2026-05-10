extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0() -> i32 {
    static mut static_i: i32 = 0_i32;;
    static mut static_f: f32 = 0.0_f32;;
    static mut static_b: bool = false;;
    static mut kX1: i32 = 1;;
    static mut kX2: i32 = 2;;
    kX1 += 1;
    return (((kX1) + (kX2)) + (static_i));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    return (((unsafe { foo_0() }) + (unsafe { foo_0() })) + (unsafe { foo_0() }));
}
