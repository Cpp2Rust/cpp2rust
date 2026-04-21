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
    let mut x1: i32 = 1;
    let mut x2: i32 = 2;
    let mut x3: i32 = 10;
    let mut x4: i32 = 20;
    let mut p1: *mut i32 = (&mut x1 as *mut i32);
    let mut p2: *mut i32 = (&mut x2 as *mut i32);
    let mut r1: i32 = (*if *&mut x1 >= *&mut x2 {
        (&mut x1) as *const _
    } else {
        (&mut x2) as *const _
    });
    let mut r2: i32 = (*if *&mut x3 <= *&mut x4 {
        (&mut x3) as *const _
    } else {
        (&mut x4) as *const _
    });
    let mut r3: i32 = (*if *p1 >= *&mut x2 {
        (p1) as *const _
    } else {
        (&mut x2) as *const _
    });
    let mut r4: i32 = (*if *p2 <= *&mut x3 {
        (p2) as *const _
    } else {
        (&mut x3) as *const _
    });
    let mut r5: i32 = {
        let mut __tmp_0 = 30;
        let mut __tmp_1 = 40;
        (*if *&mut __tmp_0 >= *&mut __tmp_1 {
            (&mut __tmp_0) as *const _
        } else {
            (&mut __tmp_1) as *const _
        })
    };
    return (((((r1) + (r2)) + (r3)) + (r4)) + (r5));
}
