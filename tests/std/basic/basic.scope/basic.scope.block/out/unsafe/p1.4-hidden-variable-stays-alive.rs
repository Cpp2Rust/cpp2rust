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
    let mut depth: i32 = 1;
    let mut outermost: *mut i32 = (&mut depth as *mut i32);
    {
        let mut depth: i32 = 2;
        (*outermost) += 10;
        assert!(((depth) == (2)));
        {
            let mut depth: i32 = 3;
            (*outermost) += 100;
            assert!(((depth) == (3)));
        }
        assert!(((depth) == (2)));
    }
    assert!(((depth) == (111)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
