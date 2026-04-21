extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn convert_without_rhs_0() {
    let mut x: i32 = 0;
    let mut y: i32 = 1;
    x = 0;
    x = ((y) + (1));
    y = 0;
    y = ((x) + (1));
    x += 1;
    y += 1;
    y = 0;
    x = 0;
    let mut z: i32 = ((x) + (y));
    z = (((x) + (y)) + (1));
    let mut arr: [i32; 2] = [1, 2];
    let mut w: i32 = ((arr[(y) as usize]) + (arr[(x) as usize]));
    w += (((z) + (y)) + (x));
    let mut arr2: [u8; 3] = [('a' as u8), ('b' as u8), ('c' as u8)];
    let mut p1: *const i32 = (&mut x as *mut i32);
    let mut c: u8 = arr2[(*p1) as usize];
    c = arr2[(*p1) as usize];
    let mut p2: *mut i32 = (&mut x as *mut i32);
    (*p2) = 1;
    let r: *mut i32 = &mut x as *mut i32;
    (*r) = 1;
}
pub unsafe fn convert_with_rhs_1() {
    let mut x: i32 = 0;
    x = ((x) + (1));
    let mut y: i32 = 0;
    y = ((y) + (1));
    let mut arr: [i32; 2] = [1, 2];
    arr[(y) as usize] = ((y) + (1));
    arr[(x) as usize] = ((x) + (1));
    arr[(x) as usize] = ((arr[(y) as usize]) + (1));
    let z: *mut i32 = &mut x as *mut i32;
    x += (*z);
    y += (*z);
    let mut p: *mut i32 = (&mut x as *mut i32);
    x += (*p);
    y += (*p);
    p = (&mut arr[(0) as usize] as *mut i32);
    arr[(0) as usize] = (*p);
    (*z) += x;
    (*z) += y;
    (*z) += (*p);
    (*p) += ((y) + (x));
    (*p) += ((x) + (*z));
    (*p) += ((y) + (*z));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { convert_without_rhs_0() });
    (unsafe { convert_with_rhs_1() });
    return 0;
}
