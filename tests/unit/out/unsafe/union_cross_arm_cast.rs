extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Copy, Clone)]
pub struct shape_a {
    pub code: u16,
    pub pad: [u8; 14],
}
#[derive(Copy, Clone)]
pub struct shape_b {
    pub code: u16,
    pub lo: u16,
    pub mid: u32,
    pub fill: [u8; 16],
    pub tail: u32,
}
#[derive(Copy, Clone)]
pub struct Container_anon_0 {
    pub a: shape_a,
    pub b: shape_b,
    pub raw_: [u8; 64],
}
#[derive(Copy, Clone, Default)]
pub struct Container {
    pub len: u32,
    pub u: Container_anon_0,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: Container = <Container>::default();
    {
        let byte_0 =
            ((&mut c as *mut Container) as *mut Container as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<Container>() as u64 {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        ((&mut c as *mut Container) as *mut Container as *mut ::libc::c_void)
    };
    c.u.a.code = 10_u16;
    c.len = (::std::mem::size_of::<shape_b>() as u64 as u32);
    (*(((&mut c.u.a as *mut shape_a) as *mut ::libc::c_void) as *mut shape_b)).tail =
        3735928559_u32;
    assert!(((c.u.b.tail) == (3735928559_u32)));
    assert!(((c.u.b.code as i32) == (10)));
    c.u.b.lo = 8080_u16;
    assert!(
        (((*((&mut c.u.raw_ as *mut [u8; 64]) as *mut u8).offset((2) as isize)) as i32) == (144))
    );
    assert!(
        (((*((&mut c.u.raw_ as *mut [u8; 64]) as *mut u8).offset((3) as isize)) as i32) == (31))
    );
    return 0;
}
