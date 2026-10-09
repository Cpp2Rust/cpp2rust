extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union basic {
    pub i: i32,
    pub f: f32,
}
impl Default for basic {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union empty {
    __empty: u8,
}
impl Default for empty {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut u: basic = <basic>::default();
    let mut e: empty = <empty>::default();
    &(e);
    u.i = 42;
    assert!(((u.i) == (42)));
    u.f = 3.14_f32;
    assert!(((u.f) == (3.14_f32)));
    let mut buf: [u8; 4] = [0_u8; 4];
    {
        let byte_0 = (buf.as_mut_ptr() as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<[u8; 4]>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        (buf.as_mut_ptr() as *mut ::libc::c_void)
    };
    let mut ru: *mut basic = (buf.as_mut_ptr() as *mut basic);
    let mut pi: *mut i32 = (&mut (*ru).i as *mut i32);
    let mut pf: *mut f32 = (&mut (*ru).f as *mut f32);
    (*ru).i = 7;
    assert!(((*pi) == (7)));
    (*pi) = 1065353216;
    assert!((((*ru).i) == (1065353216)));
    assert!(((*pf) == (1_f32)));
    assert!(((buf[(3) as usize] as i32) == (63)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
