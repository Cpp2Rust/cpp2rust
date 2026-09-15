extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn change_0(p: *mut Option<Box<i32>>) {
    let mut q: Option<Box<i32>> = Some(Box::new(7));
    (*p) = q.take();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Option<Box<i32>> = Some(Box::new(5));
    (unsafe { change_0(&mut a) });
    assert!(((*a.as_deref_mut().unwrap()) == (7)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
