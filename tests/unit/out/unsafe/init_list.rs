extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn f_0(mut list: Vec<i32>) {}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut i1: i32 = 3;
    let mut i2: i32 = 0_i32;
    let mut carr1: [i32; 2] = [1, 2];
    let mut carr2: [i32; 3] = [1, 0_i32, 0_i32];
    let mut arr: Vec<i32> = vec![1, 2, 3];
    let mut vec_: Vec<i32> = vec![1, 2, 3];
    (unsafe {
        let _list: Vec<i32> = vec![1, 2, 3, 4];
        f_0(_list)
    });
    return 0;
}
