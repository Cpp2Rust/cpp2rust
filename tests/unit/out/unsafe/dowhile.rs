extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn dowhile_0(mut x: i32) -> i32 {
    'loop_: loop {
        x += 1;
        'loop_: loop {
            x += 1;
            x += 1;
            if !((x) <= (100)) {
                break;
            }
        }
        x += 1;
        if !((x) <= (200)) {
            break;
        }
    }
    return x;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    return (unsafe {
        let _x: i32 = 0;
        dowhile_0(_x)
    });
}
