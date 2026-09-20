extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut total_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Clone, Default)]
pub struct S {
    pub v: i32,
}
impl S {
    pub unsafe fn const_method(&self) -> i32 {
        return ((self.v) * (2));
    }
    pub unsafe fn mut_method(&mut self) {
        self.v += 1;
    }
    pub unsafe fn S(mut init: i32) -> Self {
        let mut this = Self { v: init };
        (unsafe { S::mut_method(&mut this) });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_0)) +=
            (unsafe { S::const_method(&mut this) });
        this
    }
    pub unsafe fn destructor(&mut self) {
        (unsafe { S::mut_method(self) });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_0)) +=
            (unsafe { S::const_method(self) });
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub unsafe fn Point1(mut x: i32, mut y: i32) -> Self {
        let mut this = Self { x: x, y: y };
        this
    }
    pub unsafe fn Point2(mut v: i32) -> Self {
        let mut this = Point::Point1({ v }, { ((v) + (1)) });
        this.y *= 10;
        this
    }
    pub unsafe fn Point3() -> Self {
        let mut this = Point::Point2({ 4 });
        this.x += 100;
        this
    }
}
impl Default for Point {
    fn default() -> Self {
        unsafe { Point::Point3() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut s: S = S::S({ 3 });
        let _dtor_s = ScopedDestructorUnsafe::new(&raw mut s, S::destructor);
        assert!(((s.v) == (4)));
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut total_0)) == (8)));
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut total_0)) == (18)));
    let mut p: Point = Point::Point3();
    assert!(((p.x) == (104)));
    assert!(((p.y) == (50)));
    let mut q: Point = Point::Point2({ 7 });
    assert!(((q.x) == (7)));
    assert!(((q.y) == (80)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const total_0);
}
