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
    pub a: i32,
    pub b: i32,
}
pub unsafe fn sum_inner_0(mut i: *mut Inner) -> i32 {
    return (((*i).a) + ((*i).b));
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Outer_anon_0 {
    pub inner: Inner,
    pub raw_: [u8; 16],
}
impl Default for Outer_anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Outer {
    pub u: Outer_anon_0,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut standalone: Inner = <Inner>::default();
    standalone.a = 3;
    standalone.b = 4;
    assert!(
        ((((unsafe {
            let _i: *mut Inner = (&mut standalone as *mut Inner);
            sum_inner_0(_i)
        }) == (7)) as i32)
            != 0)
    );
    let mut outer: Outer = <Outer>::default();
    {
        let byte_0 = ((&mut outer as *mut Outer) as *mut Outer as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<Outer>() as u64 {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        ((&mut outer as *mut Outer) as *mut Outer as *mut ::libc::c_void)
    };
    outer.u.inner.a = 3;
    outer.u.inner.b = 4;
    assert!(
        ((((unsafe {
            let _i: *mut Inner = (&mut outer.u.inner as *mut Inner);
            sum_inner_0(_i)
        }) == (7)) as i32)
            != 0)
    );
    assert!((((((outer.u.raw_[(0) as usize] as u8) as i32) == (3)) as i32) != 0));
    assert!((((((outer.u.raw_[(4) as usize] as u8) as i32) == (4)) as i32) != 0));
    return 0;
}
