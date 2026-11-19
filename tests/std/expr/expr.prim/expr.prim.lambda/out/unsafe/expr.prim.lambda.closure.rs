extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    let mut twice: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut x: i32| -> i32 {
        return ((x) * (2));
    });
    let mut n: i32 = 4;
    assert!(((n) == (4)));
    assert!(((unsafe { twice.spec::<fn(i32) -> i32>(0).call(3,) }) == (6)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
