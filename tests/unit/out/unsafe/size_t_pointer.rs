extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn out_param_0(mut out: Option<*mut usize>) {
    let mut out: *mut usize = out.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    if !(out).is_null() {
        (*out) = 4_usize;
    }
}
pub unsafe fn parse_1(mut v: i32, mut idx: Option<*mut usize>) -> i32 {
    let mut idx: *mut usize = idx.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    if !(idx).is_null() {
        (*idx) = 3_usize;
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
    let mut v2: usize = 0_usize;
    (unsafe { out_param_0(Some((&mut v2 as *mut usize))) });
    assert!(((v2) == (4_usize)));
    let mut pidx: usize = 0_usize;
    assert!(((unsafe { parse_1(1, Some((&mut pidx as *mut usize)),) }) == (1)));
    assert!(((pidx) == (3_usize)));
    let mut sv: usize = 7_usize;
    let mut sp: *mut usize = (&mut sv as *mut usize);
    let mut spp: *mut *mut usize = (&mut sp as *mut *mut usize);
    assert!(((*(*spp)) == (7_usize)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
