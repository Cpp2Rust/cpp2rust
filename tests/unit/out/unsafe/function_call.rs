extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn function_0(mut y: i32, mut z: i32) -> i32 {
    let mut x: i32 = 5;
    return (((x) + (y)) + (z));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut y: i32 = (unsafe {
        let _y: i32 = 10;
        let _z: i32 = 1;
        function_0(_y, _z)
    });
    return y;
}
