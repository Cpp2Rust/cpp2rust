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
pub struct A {
    pub a: i32,
}
impl A {
    pub unsafe fn A(mut x: i32) -> Self {
        let mut this = Self { a: x };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct B {
    pub b: i32,
}
impl B {
    pub unsafe fn B(mut x: i32) -> Self {
        let mut this = Self { b: x };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct C {
    pub base_A: A,
    pub base_B: B,
    pub c: i32,
}
impl C {
    pub unsafe fn C(mut x: i32) -> Self {
        let mut this = Self {
            base_A: A::A({ x }),
            base_B: B::B({ ((x) + (1)) }),
            c: ((x) + (2)),
        };
        this
    }
    pub unsafe fn sum(&mut self) -> i32 {
        return ((((*(&mut self.base_A as *mut A)).a) + ((*(&mut self.base_B as *mut B)).b))
            + (self.c));
    }
}
pub unsafe fn geta_0(x: *const A) -> i32 {
    return (*x).a;
}
pub unsafe fn getb_1(mut x: *mut B) -> i32 {
    return (*x).b;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: C = C::C({ 1 });
    assert!(((unsafe { C::sum(&mut c,) }) == (6)));
    assert!(((unsafe { geta_0(&c.base_A as *const A,) }) == (1)));
    assert!(((unsafe { getb_1((&mut (*(&mut c as *mut C)).base_B as *mut B),) }) == (2)));
    let mut pb: *mut B = (&mut (*(&mut c as *mut C)).base_B as *mut B);
    (*pb).b = 10;
    assert!(((c.base_B.b) == (10)));
    assert!(((unsafe { C::sum(&mut c,) }) == (14)));
    return 0;
}
