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
    assert!(
        ((if (0..=255).contains(&(('a' as libc::c_char) as i32)) {
            ((('a' as libc::c_char) as i32) as u8).to_ascii_uppercase() as i32
        } else {
            (('a' as libc::c_char) as i32)
        }) == (('A' as libc::c_char) as i32))
    );
    assert!(
        ((if (0..=255).contains(&(('Z' as libc::c_char) as i32)) {
            ((('Z' as libc::c_char) as i32) as u8).to_ascii_uppercase() as i32
        } else {
            (('Z' as libc::c_char) as i32)
        }) == (('Z' as libc::c_char) as i32))
    );
    assert!(
        ((if (0..=255).contains(&(('5' as libc::c_char) as i32)) {
            ((('5' as libc::c_char) as i32) as u8).to_ascii_uppercase() as i32
        } else {
            (('5' as libc::c_char) as i32)
        }) == (('5' as libc::c_char) as i32))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
