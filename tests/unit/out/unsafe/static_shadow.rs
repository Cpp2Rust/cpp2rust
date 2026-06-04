extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut value_0: i32 = unsafe { 5 };
pub unsafe fn param_shadow_1(mut value: i32) -> i32 {
    return ((value) + (1));
}
pub unsafe fn local_shadow_2() -> i32 {
    let mut value: i32 = 99;
    return value;
}
pub unsafe fn read_global_3() -> i32 {
    return value_0;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((unsafe { param_shadow_1(10,) }) == (11)) as i32) != 0));
    assert!(((((unsafe { local_shadow_2() }) == (99)) as i32) != 0));
    assert!(((((unsafe { read_global_3() }) == (5)) as i32) != 0));
    return 0;
}
