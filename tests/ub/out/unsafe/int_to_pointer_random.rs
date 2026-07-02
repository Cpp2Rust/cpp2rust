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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union anon_0 {
        pub p: *mut i32,
        pub bits: u64,
    }
    impl Default for anon_0 {
        fn default() -> Self {
            unsafe { std::mem::zeroed() }
        }
    };
    let mut u: anon_0 = <anon_0>::default();
    u.bits = 3735928559_u64;
    let mut p: *mut i32 = u.p;
    return if ((((*p) == (0)) as i32) != 0) { 0 } else { 1 };
}
