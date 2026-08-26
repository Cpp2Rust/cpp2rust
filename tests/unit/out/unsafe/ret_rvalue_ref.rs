extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(v: *mut i32) -> i32 {
    return (*v);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut __tmp_0: i32 = 5;
    let i2: *mut i32 = &mut __tmp_0;
    assert!(((*i2) == (5)));
    let mut i3: i32 = (unsafe { foo_0(i2) });
    assert!(((i3) == (5)));
    assert!(
        ((unsafe {
            let mut _v: i32 = 5;
            foo_0(&mut _v)
        }) == (5))
    );
    return 0;
}
