extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    let mut total: i32 = 4;
    let mut bump: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let total: i32 = ((total) + (1));
        },
        || -> i32 {
            return total.prefix_inc();
        }
    );
    assert!(((total) == (4)));
    assert!(((unsafe { bump.call() }) == (6)));
    assert!(((unsafe { bump.call() }) == (7)));
    assert!(((total) == (4)));
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
