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
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut i1: i32 = 3;
    let mut i2: i32 = i1;
    assert!(((i1) == (3)));
    assert!(((i2) == (3)));
    let mut __tmp_0: i32 = 40;
    let i3: *mut i32 = &mut __tmp_0;
    (*i3) += 2;
    assert!(((*i3) == (42)));
    let mut __tmp_1: i32 = ((2) + (3));
    let i4: *mut i32 = &mut __tmp_1;
    assert!(((*i4) == (5)));
    let mut __tmp_2: i32 = 40;
    let i5: *const i32 = &mut __tmp_2;
    let i6: *mut i32 = i3;
    let i7: *mut i32 = i4;
    assert!(((*i6) == (*i3)));
    assert!(((*i7) == (*i4)));
    let mut i8: i32 = 3;
    let i9: *mut i32 = &mut i8;
    assert!(((*i9) == (3)));
    let mut p1: *mut i32 = (&mut i1 as *mut i32);
    let mut p2: *mut i32 = (i3);
    let mut p3: *mut i32 = (i6);
    assert!(((*p1) == (i1)));
    assert!(((*p2) == (*i3)));
    assert!(((*p3) == (*i6)));
    return 0;
}
