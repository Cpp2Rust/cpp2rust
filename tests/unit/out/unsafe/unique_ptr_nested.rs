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
pub struct Inner {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Default)]
pub struct Outer {
    pub inner: Option<Box<Inner>>,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut o: Option<Box<Outer>> = Some(Box::new(Outer {
        inner: Some(Box::new(Inner { x: 10, y: 20 })),
    }));
    (*(*o.as_deref_mut().unwrap()).inner.as_deref_mut().unwrap()).x += 5;
    let mut sum: i32 = (((*(*o.as_deref_mut().unwrap()).inner.as_deref_mut().unwrap()).x)
        + ((*(*o.as_deref_mut().unwrap()).inner.as_deref_mut().unwrap()).y));
    let mut a: Option<Box<i32>> = Some(Box::new(100));
    let mut b: Option<Box<i32>> = Some(Box::new(0));
    (*b.as_deref_mut().unwrap()) = (*a.as_deref_mut().unwrap());
    return ((sum) + (*b.as_deref_mut().unwrap()));
}
