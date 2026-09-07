extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0() -> i32 {
    static mut static_i_1: i32 = unsafe { 0_i32 };;
    static mut static_f_2: f32 = unsafe { 0.0_f32 };;
    static mut static_b_3: bool = unsafe { false };;
    static mut kX1_4: i32 = unsafe { 1 };;
    static mut kX2_5: i32 = unsafe { 2 };;
    kX1_4 += 1;
    return (((kX1_4) + (kX2_5)) + (static_i_1));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((unsafe { foo_0() }) + (unsafe { foo_0() })) + (unsafe { foo_0() })) == (15)));
    return 0;
}
