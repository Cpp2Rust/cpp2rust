extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    let mut increment_twice: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut n: i32| -> i32 {
        let mut increment: FnPtr<fn()> = lambda_unsafe!(
            {
                let n: *mut i32 = &mut n;
            },
            || {
                (*n).prefix_inc();
            }
        );
        (unsafe { increment.call() });
        (unsafe { increment.call() });
        return n;
    });
    assert!(((unsafe { increment_twice.spec::<fn(i32) -> i32>(0).call(3,) }) == (5)));
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
