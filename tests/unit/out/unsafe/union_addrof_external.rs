extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct record {
    pub code: u16,
    pub lo: u16,
    pub hi: u32,
    pub pad: [u8; 8],
}
impl Default for record {
    fn default() -> Self {
        record {
            code: 0_u16,
            lo: 0_u16,
            hi: 0_u32,
            pad: [0_u8; 8],
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Container_anon_15_3 {
    pub h: record,
    pub raw_: [u8; 128],
}
impl Default for Container_anon_15_3 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Container {
    pub view: Container_anon_15_3,
}
pub unsafe fn fill_0(mut out: *mut ::libc::c_void, mut cap: u64) {
    let mut src: [u8; 16] = [
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8,
    ];
    src[(0) as usize] = 2_u8;
    src[(1) as usize] = 0_u8;
    src[(2) as usize] = 0_u8;
    src[(3) as usize] = 80_u8;
    src[(4) as usize] = 127_u8;
    src[(5) as usize] = 0_u8;
    src[(6) as usize] = 0_u8;
    src[(7) as usize] = 1_u8;
    let mut n: u64 = if ((::std::mem::size_of::<[u8; 16]>() as u64) < (cap)) {
        ::std::mem::size_of::<[u8; 16]>() as u64
    } else {
        cap
    };
    {
        if n != 0 {
            ::std::ptr::copy_nonoverlapping(
                (src.as_mut_ptr() as *const u8 as *const ::libc::c_void),
                out,
                n as usize,
            )
        }
        out
    };
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
    (unsafe {
        let _out: *mut ::libc::c_void =
            ((&mut c.view as *mut Container_anon_15_3) as *mut ::libc::c_void);
        let _cap: u64 = ::std::mem::size_of::<Container_anon_15_3>() as u64;
        fill_0(_out, _cap)
    });
    assert!(((c.view.h.code as i32) == (2)));
    assert!((((*((&mut c.view.h.lo as *mut u16) as *mut u8).offset((0) as isize)) as i32) == (0)));
    assert!((((*((&mut c.view.h.lo as *mut u16) as *mut u8).offset((1) as isize)) as i32) == (80)));
    assert!(((c.view.raw_[(0) as usize] as i32) == (2)));
    assert!((((c.view.raw_[(3) as usize] as u8) as i32) == (80)));
    return 0;
}
