extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut s_value: i32 = unsafe { 5 };
pub unsafe fn param_shadow_0(mut value: i32) -> i32 {
    return ((value) + (1));
}
pub unsafe fn local_shadow_1() -> i32 {
    let mut value: i32 = 99;
    return value;
}
pub unsafe fn read_global_2() -> i32 {
    return s_value;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((((unsafe {
            let _value: i32 = 10;
            param_shadow_0(_value)
        }) == (11)) as i32)
            != 0)
    );
    assert!(((((unsafe { local_shadow_1() }) == (99)) as i32) != 0));
    assert!(((((unsafe { read_global_2() }) == (5)) as i32) != 0));
    return 0;
}
