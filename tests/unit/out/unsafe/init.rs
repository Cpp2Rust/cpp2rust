extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Copy, Clone, Default)]
pub struct X {
    pub x: i32,
}
pub unsafe fn func_0() -> i32 {
    return 42;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 0_i32;
    let mut p: *mut i32 = Default::default();
    let g: *mut i32 = &mut x as *mut i32;
    let mut q: *mut i32 = (&mut x as *mut i32);
    let mut z: *mut i32 = p;
    let mut xx: X = <X>::default();
    let mut zz: *mut X = (&mut xx as *mut X);
    xx.x = 1;
    q = (&mut xx.x as *mut i32);
    q = (&mut (*zz).x as *mut i32);
    (*zz).x = 2;
    let mut ww: X = xx.clone();
    ww = xx;
    let mut aa: i32 = (unsafe { func_0() });
    aa = (unsafe { func_0() });
    return 3;
}
