extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct X {
    pub x: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Y {
    pub x: X,
    pub p: *mut X,
}
impl Y {
    pub unsafe fn foo(&mut self) -> *mut X {
        return &mut self.x as *mut X;
    }
    pub unsafe fn ptr(&mut self) -> *mut X {
        return (&mut self.x as *mut X);
    }
}
// SAFETY: preserves unsafe C semantics; thread-safety is not enforced
unsafe impl Sync for Y {}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x1: i32 = 5;
    let mut x2: i32 = x1;
    let mut x3: i32 = ((x1) + (5));
    let mut x4: i32 = ((x3) + (x2));
    x1 = 5;
    x2 = x1;
    x3 = ((x1) + (5));
    x4 = ((x3) + (x2));
    let mut p1: *mut i32 = (&mut x1 as *mut i32);
    p1 = (&mut x2 as *mut i32);
    (*p1) = x1;
    (*p1) = (((x1) + (x4)) + (1));
    let mut x5: i32 = (*p1);
    let mut x6: i32 = (((*p1) + (x3)) + (5));
    let r: *mut i32 = &mut x1 as *mut i32;
    (*r) = 5;
    (*r) = ((*p1) + (5));
    let mut x7: i32 = (*r);
    let mut x8: i32 = (((*r) + (x1)) + (5));
    let mut p2: *mut i32 = (r);
    let mut x: X = X { x: 1 };
    let mut y: Y = Y {
        x: X { x: 0 },
        p: (&mut x as *mut X),
    };
    y.x.x = 5;
    (*(unsafe { y.foo() })).x = 1;
    (*y.p).x = 10;
    let mut p3: *mut Y = (&mut y as *mut Y);
    (*(*p3).p).x = 100;
    (*(unsafe { y.ptr() })).x = 1;
    (*(unsafe { y.ptr() })).x = 50;
    return x.x;
}
