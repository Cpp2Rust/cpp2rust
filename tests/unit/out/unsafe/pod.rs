extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Copy, Clone, Default)]
pub struct POD {
    pub x1: i32,
    pub x2: i32,
    pub x3: i32,
}
pub unsafe fn PODIncrement_0(pod: *mut POD) {
    (*pod).x1 += 1;
    (*pod).x2 += 2;
    (*pod).x3 += 3;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p1: POD = POD {
        x1: 10,
        x2: 11,
        x3: 12,
    };
    let mut p2: POD = POD {
        x1: p1.x1,
        x2: p1.x2,
        x3: p1.x3,
    };
    (unsafe {
        let _pod: *mut POD = &mut p2 as *mut POD;
        PODIncrement_0(_pod)
    });
    return (((p2.x1) + (p2.x2)) + (p2.x3));
}
