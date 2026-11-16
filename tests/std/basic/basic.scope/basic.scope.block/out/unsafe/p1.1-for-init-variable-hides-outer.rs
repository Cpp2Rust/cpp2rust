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
    let mut index: i32 = 42;
    let mut iterations: i32 = 0;
    let mut sum: i32 = 0;
    {
        let mut index: i32 = 0;
        'loop_: while ((index) < (3)) {
            iterations.prefix_inc();
            sum += index;
            index.prefix_inc();
        }
    }
    assert!(((iterations) == (3)));
    assert!(((sum) == (3)));
    assert!(((index) == (42)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
