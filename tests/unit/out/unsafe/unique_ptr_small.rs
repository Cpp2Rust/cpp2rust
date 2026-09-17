extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn change_0(n: *mut Option<Box<i32>>) {
    let mut m: Option<Box<i32>> = Some(Box::new(20));
    (*n) = m.take();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut n: Option<Box<i32>> = Some(Box::new(10));
    (unsafe { change_0(&mut n) });
    assert!(((*n.as_deref_mut().unwrap()) == (20)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
