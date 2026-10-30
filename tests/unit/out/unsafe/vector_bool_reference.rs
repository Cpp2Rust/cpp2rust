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
    let mut v1: Vec<bool> = Vec::from([true]);
    let mut r1: *mut bool = (&mut (v1)[0_usize as usize] as *mut bool);
    assert!((*(&mut (v1)[0_usize as usize] as *mut bool)) && (*r1));
    {
        *r1 = false;
        r1
    };
    assert!(!(*(&mut (v1)[0_usize as usize] as *mut bool)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
