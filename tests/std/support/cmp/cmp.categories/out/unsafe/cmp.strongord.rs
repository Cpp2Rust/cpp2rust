extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn less_than_zero_0(mut o: std::cmp::Ordering) -> i32 {
    return ((o == std::cmp::Ordering::Less) as i32);
}
pub unsafe fn test_1() {
    let mut o: std::cmp::Ordering = std::cmp::Ord::cmp(&(1), &(2));
    let mut a: i32 = ((o == std::cmp::Ordering::Less) as i32);
    assert!(((a) == (1)));
    assert!(((unsafe { less_than_zero_0(o,) }) == (1)));
    assert!((((o == std::cmp::Ordering::Less) as i32) == (1)));
    let mut d: i64 = ((o == std::cmp::Ordering::Greater) as i64);
    assert!(((d) == (0_i64)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_1() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
