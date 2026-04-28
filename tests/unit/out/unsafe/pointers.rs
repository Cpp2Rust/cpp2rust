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
pub struct Test {
    pub x: i32,
}
impl Test {
    pub unsafe fn inc(&mut self) {
        self.x.postfix_inc();
    }
    pub unsafe fn dec(&mut self) {
        self.x.postfix_dec();
    }
    pub unsafe fn as_ptr(&mut self) -> *mut i32 {
        return (&mut self.x as *mut i32);
    }
    pub unsafe fn update(&mut self, mut x: i32, mut y: i32) {
        self.x = ((x) + (y));
    }
}
pub unsafe fn Update_0(mut t: *mut Test) -> *mut Test {
    let mut x: i32 = 1;
    let mut y: i32 = 2;
    x.prefix_inc();
    (unsafe {
        let _x: i32 = x;
        let _y: i32 = y;
        (*t).update(_x, _y)
    });
    x = (*t).x;
    y = (*t).x;
    (unsafe {
        let _x: i32 = x;
        let _y: i32 = y;
        (*t).update(_x, _y)
    });
    return t;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut t1: Test = Test { x: 100 };
    let mut t2: *mut Test = (unsafe {
        let _t: *mut Test = (&mut t1 as *mut Test);
        Update_0(_t)
    });
    let mut t3: *mut Test = Default::default();
    t3 = t2;
    (*t3).x = 15;
    (*(unsafe { (*t3).as_ptr() })) += 10;
    return ((((*t3).x) + ((*t2).x)) + (t1.x));
}
