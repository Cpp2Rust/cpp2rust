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
    let a: *const i32 = {
        let mut __tmp_0 = 1;
        let mut __tmp_1 = 2;
        if *&mut __tmp_0 <= *&mut __tmp_1 {
            (&mut __tmp_0) as *const _
        } else {
            (&mut __tmp_1) as *const _
        }
    };
    let b: *const i32 = {
        let mut __tmp_0 = 1;
        let mut __tmp_1 = 2;
        if *&mut __tmp_0 >= *&mut __tmp_1 {
            (&mut __tmp_0) as *const _
        } else {
            (&mut __tmp_1) as *const _
        }
    };
    return (((*a) == (*b)) as i32);
}
