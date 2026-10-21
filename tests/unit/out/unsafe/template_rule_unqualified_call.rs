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
        (({
            let __c: i32 = (('a' as libc::c_char) as i32).into();
            if (0..=255).contains(&__c) {
                (__c as u8).to_ascii_uppercase() as i32
            } else {
                __c
            }
        }) == (('A' as libc::c_char) as i32))
    );
    assert!(
        (({
            let __c: i32 = (('Z' as libc::c_char) as i32).into();
            if (0..=255).contains(&__c) {
                (__c as u8).to_ascii_uppercase() as i32
            } else {
                __c
            }
        }) == (('Z' as libc::c_char) as i32))
    );
    assert!(
        (({
            let __c: i32 = (('5' as libc::c_char) as i32).into();
            if (0..=255).contains(&__c) {
                (__c as u8).to_ascii_uppercase() as i32
            } else {
                __c
            }
        }) == (('5' as libc::c_char) as i32))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
