extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn classify_0(mut x: *mut i32) -> i32 {
    {
        return (*x);
    }
    return 1;
}
pub unsafe fn classify_1(mut x: i64) -> i32 {
    {
        {
            return 2;
        }
    }
    return 1;
}
pub unsafe fn classify_2(mut x: i32) -> i32 {
    {
        {}
    }
    return 1;
}
pub unsafe fn keep_both_3(mut x: i32) -> i32 {
    {
        return ((x) + (1));
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn widen_4(mut v: u32) -> u64 {
    {}
    return (v as u64);
}
pub unsafe fn widen_5(mut v: u64) -> u64 {
    {
        let mut hi: u64 = v;
        hi <<= 32;
        return hi;
    }
    return v;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: i32 = 7;
    assert!(((unsafe { classify_0((&mut v as *mut i32),) }) == (7)));
    assert!(((unsafe { classify_1(1_i64,) }) == (2)));
    assert!(((unsafe { classify_2(1,) }) == (1)));
    assert!(((unsafe { keep_both_3(1,) }) == (2)));
    assert!(((unsafe { widen_4(1_u32,) }) == (1_u64)));
    assert!(((unsafe { widen_5(1_u64,) }) == (4294967296_u64)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
