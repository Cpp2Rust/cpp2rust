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
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 5;
    let mut p1: *const i32 = (&mut x as *mut i32);
    let mut p2: *const i32 = (&mut x as *mut i32);
    assert!(((p1) == (p2)));
    let mut y: i32 = 5;
    let mut p3: *const i32 = (&mut y as *mut i32);
    assert!(((p1) != (p3)));
    let mut arr: [i32; 3] = [1, 2, 3];
    let mut p: *mut i32 = arr.as_mut_ptr();
    assert!(((p) == (arr.as_mut_ptr())));
    assert!(((p.offset((1) as isize)) == (&mut arr[(1) as usize] as *mut i32)));
    assert!(((p.offset((2) as isize)) == (&mut arr[(2) as usize] as *mut i32)));
    let mut val: i32 = 42;
    let mut orig: *mut i32 = (&mut val as *mut i32);
    let mut as_bytes: *mut u8 = (orig as *mut u8);
    let mut back: *mut i32 = (as_bytes as *mut i32);
    assert!(((orig) == (back)));
    let mut arr_bytes: *mut u8 = (arr.as_mut_ptr() as *mut u8);
    let mut arr_back: *mut i32 = (arr_bytes as *mut i32);
    assert!(((arr_back) == (arr.as_mut_ptr())));
    assert!(((arr_back.offset((1) as isize)) == (&mut arr[(1) as usize] as *mut i32)));
    return 0;
}
