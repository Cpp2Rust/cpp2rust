extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    let l: *const std::cmp::Ordering = &libcc2rs::STRONG_ORDERING_LESS;
    let e: *const std::cmp::Ordering = &libcc2rs::STRONG_ORDERING_EQUAL;
    let g: *const std::cmp::Ordering = &libcc2rs::STRONG_ORDERING_GREATER;
    assert!(((l) == (&raw const libcc2rs::STRONG_ORDERING_LESS as *const std::cmp::Ordering)));
    assert!(((e) == (&raw const libcc2rs::STRONG_ORDERING_EQUAL as *const std::cmp::Ordering)));
    assert!(((g) == (&raw const libcc2rs::STRONG_ORDERING_GREATER as *const std::cmp::Ordering)));
    assert!((*l) == std::cmp::Ordering::Less);
    assert!((*e) == std::cmp::Ordering::Equal);
    assert!((*g) == std::cmp::Ordering::Greater);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
