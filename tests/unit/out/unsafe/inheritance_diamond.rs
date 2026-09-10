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
    pub base_A: A,
}
impl B {
    pub unsafe fn B(mut x: i32) -> Self {
        let mut this = Self {
            base_A: A::A({ x }),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct C {
    pub base_A: A,
}
impl C {
    pub unsafe fn C(mut x: i32) -> Self {
        let mut this = Self {
            base_A: A::A({ ((x) + (1)) }),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct D {
    pub base_B: B,
    pub base_C: C,
}
impl D {
    pub unsafe fn D(mut x: i32) -> Self {
        let mut this = Self {
            base_B: B::B({ x }),
            base_C: C::C({ x }),
        };
        this
    }
    pub unsafe fn sum(&mut self) -> i32 {
        return (((*(&mut (*(&mut self.base_B as *mut B)).base_A as *mut A)).a)
            + ((*(&mut (*(&mut self.base_C as *mut C)).base_A as *mut A)).a));
    }
}
pub unsafe fn geta_0(x: *const A) -> i32 {
    return (*x).a;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: D = D::D({ 1 });
    assert!(((unsafe { D::sum(&mut d,) }) == (3)));
    let b: *mut B = &mut d.base_B as *mut B;
    let c: *mut C = &mut d.base_C as *mut C;
    assert!(((unsafe { geta_0((&(*b).base_A as *const A),) }) == (1)));
    assert!(((unsafe { geta_0((&(*c).base_A as *const A),) }) == (2)));
    (*c).base_A.a = 5;
    assert!(((unsafe { D::sum(&mut d,) }) == (6)));
    assert!(((&mut (*b).base_A as *mut A) != (&mut (*c).base_A as *mut A)));
    return 0;
}
