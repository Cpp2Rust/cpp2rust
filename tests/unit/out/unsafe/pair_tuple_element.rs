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
    let mut p: (f64, i32) = (1.5_f64, 2);
    let mut first: f64 = *p.get_mut::<0>();
    let mut second: i32 = *p.get_mut::<1>();
    let ref_: *mut i32 = &mut *p.get_mut::<1>();
    (*ref_) = 7;
    assert!(((first) == (1.5_f64)));
    assert!(((second) == (2)));
    assert!(((p.1) == (7)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
