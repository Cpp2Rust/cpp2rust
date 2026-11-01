extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub unsafe fn new(mut x: i32, mut y: i32) -> Self {
        let mut this = Self { x: x, y: y };
        this
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Option<Box<dyn std::any::Any>> = Some(Box::new(Point::new({ 3 }, { 4 })));
    assert!(a.is_some());
    let mut p: *mut Point = match (*(&mut a as *mut Option<Box<dyn std::any::Any>>))
        .as_mut()
        .and_then(|__b| __b.downcast_mut::<Point>())
    {
        Some(__v) => __v as *mut Point,
        None => std::ptr::null_mut(),
    };
    assert!(!((p).is_null()));
    assert!((((*p).x) == (3)));
    assert!((((*p).y) == (4)));
    assert!(
        (match (*(&mut a as *mut Option<Box<dyn std::any::Any>>))
            .as_mut()
            .and_then(|__b| __b.downcast_mut::<i32>())
        {
            Some(__v) => __v as *mut i32,
            None => std::ptr::null_mut(),
        })
        .is_null()
    );
    let q: *mut Point = {
        a = Some(Box::new(Point::new({ 5 }, { 6 })));
        a.as_mut().unwrap().downcast_mut::<Point>().unwrap()
    };
    assert!((((*q).x) == (5)));
    assert!(
        (((*match (*(&mut a as *mut Option<Box<dyn std::any::Any>>))
            .as_mut()
            .and_then(|__b| __b.downcast_mut::<Point>())
        {
            Some(__v) => __v as *mut Point,
            None => std::ptr::null_mut(),
        })
        .y) == (6))
    );
    let mut b: Option<Box<dyn std::any::Any>> = Some(Box::new(7));
    assert!(
        ((*match (*(&mut b as *mut Option<Box<dyn std::any::Any>>))
            .as_mut()
            .and_then(|__b| __b.downcast_mut::<i32>())
        {
            Some(__v) => __v as *mut i32,
            None => std::ptr::null_mut(),
        }) == (7))
    );
    b = None;
    assert!(!(b.is_some()));
    assert!(
        (match (*(&mut b as *mut Option<Box<dyn std::any::Any>>))
            .as_mut()
            .and_then(|__b| __b.downcast_mut::<i32>())
        {
            Some(__v) => __v as *mut i32,
            None => std::ptr::null_mut(),
        })
        .is_null()
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
