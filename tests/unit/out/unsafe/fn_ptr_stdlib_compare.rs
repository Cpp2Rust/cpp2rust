extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn my_alternative_fread_0(
    mut p: *mut u8,
    mut n: u64,
    mut m: u64,
    mut f: *mut ::libc::c_void,
) -> u64 {
    return 22_u64;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut fn1: Option<unsafe fn(*mut ::libc::c_void, u64, u64, *mut ::std::fs::File) -> u64> =
        Some(rules::stdio_tgt_unsafe::f5);
    assert!(((fn1) == (Some(rules::stdio_tgt_unsafe::f5))));
    assert!(!((fn1).is_none()));
    let mut fn2: Option<unsafe fn(*mut u8, u64, u64, *mut ::libc::c_void) -> u64> =
        std::mem::transmute::<
            Option<unsafe fn(*mut ::libc::c_void, u64, u64, *mut ::std::fs::File) -> u64>,
            Option<unsafe fn(*mut u8, u64, u64, *mut ::libc::c_void) -> u64>,
        >(Some(rules::stdio_tgt_unsafe::f5));
    assert!(
        ((fn1)
            == (std::mem::transmute::<
                Option<unsafe fn(*mut u8, u64, u64, *mut ::libc::c_void) -> u64>,
                Option<unsafe fn(*mut ::libc::c_void, u64, u64, *mut ::std::fs::File) -> u64>,
            >(fn2)))
    );
    let mut f3: Option<unsafe fn(*mut ::libc::c_void, u64, u64, *mut ::std::fs::File) -> u64> =
        std::mem::transmute::<
            Option<unsafe fn(*mut u8, u64, u64, *mut ::libc::c_void) -> u64>,
            Option<unsafe fn(*mut ::libc::c_void, u64, u64, *mut ::std::fs::File) -> u64>,
        >(Some(my_alternative_fread_0));
    assert!(
        ((unsafe {
            let _arg0: *mut ::libc::c_void = Default::default();
            let _arg1: u64 = 0_u64;
            let _arg2: u64 = 0_u64;
            let _arg3: *mut ::std::fs::File = Default::default();
            (f3).unwrap()(_arg0, _arg1, _arg2, _arg3)
        }) == (22_u64))
    );
    return 0;
}
