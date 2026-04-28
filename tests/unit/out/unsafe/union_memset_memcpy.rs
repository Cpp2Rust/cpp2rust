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
    pub hi: u32,
    pub fill: [u8; 8],
}
#[derive(Copy, Clone)]
pub struct Container_anon_0 {
    pub a: shape_a,
    pub b: shape_b,
    pub raw_: [u8; 256],
}
#[derive(Copy, Clone, Default)]
pub struct Container {
    pub view: Container_anon_0,
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
    assert!(((c.view.a.code as i32) == (0)));
    assert!(((c.view.b.lo as i32) == (0)));
    assert!(((c.view.raw_[(0) as usize] as i32) == (0)));
    assert!(((c.view.raw_[(255) as usize] as i32) == (0)));
    let mut src: [u8; 16] = [
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8,
    ];
    src[(0) as usize] = 2_u8;
    src[(2) as usize] = 80_u8;
    src[(3) as usize] = 0_u8;
    src[(4) as usize] = 127_u8;
    src[(5) as usize] = 0_u8;
    src[(6) as usize] = 0_u8;
    src[(7) as usize] = 1_u8;
    let mut len: u64 = 16_u64;
    assert!(((len) <= (::std::mem::size_of::<[u8; 256]>() as u64)));
    {
        if len != 0 {
            ::std::ptr::copy_nonoverlapping(
                (src.as_mut_ptr() as *const u8 as *const ::libc::c_void),
                ((&mut c.view.raw_ as *mut [u8; 256]) as *mut [u8; 256] as *mut ::libc::c_void),
                len as usize,
            )
        }
        ((&mut c.view.raw_ as *mut [u8; 256]) as *mut [u8; 256] as *mut ::libc::c_void)
    };
    assert!(((c.view.b.code as i32) == (2)));
    assert!((((*((&mut c.view.b.lo as *mut u16) as *mut u8).offset((0) as isize)) as i32) == (80)));
    {
        let byte_0 =
            ((&mut c as *mut Container) as *mut Container as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<Container>() as u64 {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        ((&mut c as *mut Container) as *mut Container as *mut ::libc::c_void)
    };
    assert!(((c.view.b.code as i32) == (0)));
    return 0;
}
