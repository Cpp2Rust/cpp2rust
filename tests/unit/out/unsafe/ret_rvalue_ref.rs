extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(v: i32) -> i32 {
    return (*v);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let i2: i32 = &mut 5 as i32;
    assert!(((*i2) == (5)));
    let mut i3: i32 = (unsafe { foo_0(i2) });
    assert!(((i3) == (5)));
    assert!(((unsafe { foo_0(&mut 5 as i32,) }) == (5)));
    return 0;
}
