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
    let mut amount: i32 = 6;
    let mut total: i32 = 0;
    let mut add_doubled: FnPtr<fn(i32) -> i32> = lambda_unsafe!(
        {
            let total: *mut i32 = &mut total;
        },
        |mut amount: i32| -> i32 {
            amount *= 2;
            (*total) += amount;
            return amount;
        }
    );
    assert!(((unsafe { add_doubled.call(4,) }) == (8)));
    assert!(((total) == (8)));
    assert!(((amount) == (6)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
