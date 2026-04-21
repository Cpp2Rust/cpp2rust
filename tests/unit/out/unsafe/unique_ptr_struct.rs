extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Copy, Clone, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
pub unsafe fn sum_0(mut p: Point) -> i32 {
    return ((p.x) + (p.y));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: Option<Box<Point>> = Some(Box::new(Point { x: 3, y: 4 }));
    (*p.as_deref_mut().unwrap()).x += 10;
    (*p.as_deref_mut().unwrap()).y =
        (((*p.as_deref_mut().unwrap()).x) + ((*p.as_deref_mut().unwrap()).y));
    let mut s: i32 = (unsafe {
        let _p: Point = (*p.as_deref_mut().unwrap()).clone();
        sum_0(_p)
    });
    return s;
}
