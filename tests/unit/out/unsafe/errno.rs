extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_errno_0() {
    (*libcc2rs::cpp2rust_errno()) = 0;
    assert!(((((*libcc2rs::cpp2rust_errno()) == (0)) as i32) != 0));
    (*libcc2rs::cpp2rust_errno()) = 42;
    assert!(((((*libcc2rs::cpp2rust_errno()) == (42)) as i32) != 0));
    let mut saved: i32 = (*libcc2rs::cpp2rust_errno());
    assert!(((((saved) == (42)) as i32) != 0));
    (*libcc2rs::cpp2rust_errno()) = 0;
}
pub unsafe fn test_errno_preserved_across_strdup_1() {
    (*libcc2rs::cpp2rust_errno()) = 99;
    let mut d: *mut u8 =
        libc::strdup((b"hello\0".as_ptr().cast_mut()).cast_const() as *const i8) as *mut u8;
    assert!((((!((d).is_null())) as i32) != 0));
    assert!(((((*libcc2rs::cpp2rust_errno()) == (99)) as i32) != 0));
    libc::free((d as *mut u8 as *mut ::libc::c_void));
    (*libcc2rs::cpp2rust_errno()) = 0;
}
pub unsafe fn test_errno_from_fseek_2() {
    (*libcc2rs::cpp2rust_errno()) = 0;
    let mut r: i32 = libc::fseek(libcc2rs::stdin_unsafe(), 0_i64 as ::libc::c_long, 0);
    assert!(((((r) == (-1_i32)) as i32) != 0));
    assert!(((((*libcc2rs::cpp2rust_errno()) == (29)) as i32) != 0));
    (*libcc2rs::cpp2rust_errno()) = 0;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_errno_0() });
    (unsafe { test_errno_preserved_across_strdup_1() });
    (unsafe { test_errno_from_fseek_2() });
    return 0;
}
