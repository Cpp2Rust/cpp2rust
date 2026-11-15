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
    let n: i32 = 5;
    let mut f: FnPtr<fn() -> *const i32> = lambda_unsafe!(
        {
            let n: i32 = n;
        },
        || -> *const i32 {
            return (&n as *const i32);
        }
    );
    assert!(((unsafe { f.call() }) != (&n as *const i32)));
    assert!(((*(unsafe { f.call() })) == (5)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
