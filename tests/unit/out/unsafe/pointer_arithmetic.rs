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
    let mut x: i32 = 1;
    let mut p: *mut i32 = (&mut x as *mut i32);
    (*p) += 1;
    if ((x) == (2)) {
        let mut a: [i32; 2] = [1, 2];
        p = (&mut a[(1) as usize] as *mut i32);
        (*p) += 1;
        if ((a[(0) as usize]) == (1)) && ((a[(1) as usize]) == (3)) {
            p.prefix_dec();
            (*p) += 1;
            if ((a[(0) as usize]) == (2)) && ((a[(1) as usize]) == (3)) {
                p = (&mut x as *mut i32);
                (*p) += 1;
                if ((x) == (3)) {
                    let mut p2: *mut i32 = p;
                    (*p2) += 1;
                    return (((x) == (4)) as i32);
                }
            }
        }
    }
    return -1_i32;
}
