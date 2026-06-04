extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn dowhile_0(mut x: i32) -> i32 {
    let mut __do_while = true;
    'loop_: while __do_while || ((x) <= (200)) {
        __do_while = false;
        x += 1;
        let mut __do_while = true;
        'loop_: while __do_while || ((x) <= (100)) {
            __do_while = false;
            x += 1;
            x += 1;
        }
        x += 1;
    }
    return x;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    return (unsafe { dowhile_0(0) });
}
