extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut rfn: Option<unsafe fn(*mut ::libc::c_void, u64, u64, *mut ::std::fs::File) -> u64> =
        Some(rules::stdio_tgt_unsafe::f5 as _);
    assert!(((rfn) == (Some(rules::stdio_tgt_unsafe::f5 as _))));
    assert!(!((rfn).is_none()));
    let mut rfn2: Option<unsafe fn(*mut u8, u64, u64, *mut ::libc::c_void) -> u64> =
        std::mem::transmute::<
            Option<unsafe fn(*mut ::libc::c_void, u64, u64, *mut ::std::fs::File) -> u64>,
            Option<unsafe fn(*mut u8, u64, u64, *mut ::libc::c_void) -> u64>,
        >(Some(rules::stdio_tgt_unsafe::f5 as _));
    assert!(
        ((rfn)
            == (std::mem::transmute::<
                Option<unsafe fn(*mut u8, u64, u64, *mut ::libc::c_void) -> u64>,
                Option<unsafe fn(*mut ::libc::c_void, u64, u64, *mut ::std::fs::File) -> u64>,
            >(rfn2)))
    );
    return 0;
}
