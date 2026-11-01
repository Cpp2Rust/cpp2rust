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
    assert!(((libc::close(1)) == (0)));
    write!((*libcc2rs::cout_unsafe()), "not written\n",);
    write!((*libcc2rs::cerr_unsafe()), "still open\n",);
    assert!(((libc::close(2)) == (0)));
    write!((*libcc2rs::cerr_unsafe()), "not written either\n",);
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
