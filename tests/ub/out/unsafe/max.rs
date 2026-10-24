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
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let a: *const i32 = {
        let mut __tmp_0: i32 = 1;
        let mut __tmp_1: i32 = 2;
        {
            let __a: *const _ = &mut __tmp_0;
            let __b: *const _ = &mut __tmp_1;
            if *__b < *__a { __b } else { __a }
        }
    };
    let b: *const i32 = {
        let mut __tmp_2: i32 = 1;
        let mut __tmp_3: i32 = 2;
        {
            let __a: *const _ = &mut __tmp_2;
            let __b: *const _ = &mut __tmp_3;
            if *__a < *__b { __b } else { __a }
        }
    };
    return (((*a) == (*b)) as i32);
}
pub unsafe fn __cpp2rust_init_globals() {}
