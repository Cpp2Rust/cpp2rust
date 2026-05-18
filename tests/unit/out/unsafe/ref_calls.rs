extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn bar_0() -> i32 {
    return 1;
}
pub unsafe fn foo_1(x: *mut i32) -> *mut i32 {
    return x;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 5;
    let mut y: i32 = (*(unsafe {
        let _x: *mut i32 = &mut x as *mut i32;
        foo_1(_x)
    }));
    let z: *mut i32 = (unsafe {
        let _x: *mut i32 = &mut x as *mut i32;
        foo_1(_x)
    });
    return ((((*(unsafe {
        let _x: *mut i32 = &mut x as *mut i32;
        foo_1(_x)
    })) + (*(unsafe {
        let _x: *mut i32 = &mut y as *mut i32;
        foo_1(_x)
    }))) + (*(unsafe {
        let _x: *mut i32 = z;
        foo_1(_x)
    }))) + (unsafe { bar_0() }));
}
