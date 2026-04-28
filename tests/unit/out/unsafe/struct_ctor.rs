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
pub struct StructWithCtor {
    x1_: i32,
    x2_: i32,
}
impl StructWithCtor {
    pub unsafe fn StructWithCtor(mut x1: i32, mut x2: i32) -> Self {
        let mut this = Self { x1_: x1, x2_: x2 };
        this.x1_.prefix_inc();
        this.x2_.prefix_dec();
        this
    }
    pub unsafe fn x1(&self) -> *const i32 {
        return &self.x1_ as *const i32;
    }
    pub unsafe fn x2(&self) -> *const i32 {
        return &self.x2_ as *const i32;
    }
}
pub unsafe fn foo_0(x: *mut i32) -> *mut i32 {
    return x;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut struct_with_ctor: StructWithCtor = StructWithCtor::StructWithCtor(1, 2);
    let mut x: i32 = 3;
    return (((((*(unsafe {
        let _x: *mut i32 = &mut x as *mut i32;
        foo_0(_x)
    })) == (3))
        && ((*(unsafe { struct_with_ctor.x1() })) == (2)))
        && ((*(unsafe { struct_with_ctor.x2() })) == (1))) as i32);
}
