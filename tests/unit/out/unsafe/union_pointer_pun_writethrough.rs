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
    let mut x: i64 = (-1_i32 as i64);
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union anon_0 {
        pub as_unsigned: *mut u64,
        pub as_signed: *mut i64,
    }
    impl Default for anon_0 {
        fn default() -> Self {
            unsafe { std::mem::zeroed() }
        }
    };
    let mut pp: anon_0 = <anon_0>::default();
    pp.as_signed = (&mut x as *mut i64);
    (*pp.as_unsigned) = 42;
    assert!(((((x) == (42_i64)) as i32) != 0));
    return 0;
}
