extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(mut a: i32, mut b: Option<i32>) -> i32 {
    let mut b: i32 = b.unwrap_or(10);
    return ((a) + (b));
}
pub unsafe fn baz_1(mut a: *mut i32, mut b: Option<*mut i32>) -> bool {
    let mut b: *mut i32 = b.unwrap_or(std::ptr::null_mut());
    return ((a) == (b));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { foo_0(1, None,) }) == (11)));
    assert!(((unsafe { foo_0(1, Some(2),) }) == (3)));
    let mut a: i32 = 0;
    assert!((((unsafe { baz_1((&mut a as *mut i32), None,) }) as i32) == (false as i32)));
    assert!(
        (((unsafe {
            let _a: *mut i32 = (&mut a as *mut i32);
            let _b: *mut i32 = (&mut a as *mut i32);
            baz_1(_a, Some(_b))
        }) as i32)
            == (true as i32))
    );
    return 0;
}
