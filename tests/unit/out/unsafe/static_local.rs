extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0() -> i32 {
    static mut s_static_i: i32 = unsafe { 0_i32 };;
    static mut s_static_f: f32 = unsafe { 0.0_f32 };;
    static mut s_static_b: bool = unsafe { false };;
    static mut s_kX1: i32 = unsafe { 1 };;
    static mut s_kX2: i32 = unsafe { 2 };;
    s_kX1 += 1;
    return (((s_kX1) + (s_kX2)) + (s_static_i));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    return (((unsafe { foo_0() }) + (unsafe { foo_0() })) + (unsafe { foo_0() }));
}
