extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn second_of_0(p: *const (f64, i32)) -> i32 {
    return *(*p).get::<1_u64>();
}
pub unsafe fn first_of_1(p: *const (f64, i32)) -> f64 {
    return *(*p).get::<0_u64>();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: (f64, i32) = (1.5_f64, 2);
    assert!(((*p.get_mut::<0_u64>()) == (1.5_f64)));
    assert!(((*p.get_mut::<1_u64>()) == (2)));
    assert!(((unsafe { first_of_1(&p,) }) == (1.5_f64)));
    assert!(((unsafe { second_of_0(&p,) }) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
