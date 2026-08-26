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
    let i3: i32 = &mut 40 as i32;
    (*i3) += 2;
    assert!(((*i3) == (42)));
    let i4: i32 = &mut ((2) + (3)) as i32;
    assert!(((*i4) == (5)));
    let i5: *const i32 = &40 as *const i32;
    let i6: *mut i32 = i3;
    let i7: *mut i32 = i4;
    assert!(((*i6) == (*i3)));
    assert!(((*i7) == (*i4)));
    let mut p1: *mut i32 = (&mut i1 as *mut i32);
    let mut p2: *mut i32 = (i3);
    let mut p3: *mut i32 = (i6);
    assert!(((*p1) == (i1)));
    assert!(((*p2) == (*i3)));
    assert!(((*p3) == (*i6)));
    return 0;
}
