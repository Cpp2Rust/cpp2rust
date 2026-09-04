extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sum_0(mut args_0: i32, mut args_1: i32, mut args_2: i32) -> i32 {
    return ((args_0) + ((args_1) + ((args_2) + (0))));
}
pub unsafe fn sum_1(mut args_0: i32, mut args_1: i32) -> i32 {
    return ((args_0) + ((args_1) + (0)));
}
pub unsafe fn sum_2(mut args: i32) -> i32 {
    return ((args) + (0));
}
pub unsafe fn first_3(mut x: i32, mut args_1: i32, mut args_2: i32) -> i32 {
    return ((x) + (unsafe { sum_1(args_1, args_2) }));
}
pub unsafe fn first_4(mut x: i32, mut args: i32) -> i32 {
    return ((x) + (unsafe { sum_2(args) }));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { sum_0(1, 2, 3,) }) == (6)));
    assert!(((unsafe { sum_1(4, 5,) }) == (9)));
    assert!(((unsafe { first_3(10, 1, 2,) }) == (13)));
    assert!(((unsafe { first_4(10, 0,) }) == (10)));
    return 0;
}
