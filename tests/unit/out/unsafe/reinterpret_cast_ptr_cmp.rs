extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct W {
    pub h: i32,
    pub arr: [S; 3],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct P {
    pub a: *mut i32,
    pub arr: [*mut i32; 3],
}
pub unsafe fn typed_0() {
    let mut w: W = <W>::default();
    let mut p: *mut S = (&mut w.arr[(2) as usize] as *mut S);
    let mut q: *mut S = ((((&mut w as *mut W) as *mut libc::c_char)
        .offset((::std::mem::size_of::<i32>()) as isize)
        .offset(((2_usize).wrapping_mul((::std::mem::size_of::<S>() as usize))) as isize))
        as *mut S);
    assert!(((p) == (q)));
    assert!(((&mut w.arr[(1) as usize] as *mut S) < (q)));
    assert!(
        ((((q as usize - (&mut w.arr[(0) as usize] as *mut S) as usize)
            / ::std::mem::size_of::<S>()) as i64)
            == (2_i64))
    );
    let mut v: i32 = 0;
    let mut ptrs: [*mut i32; 4] = [
        (&mut v as *mut i32),
        (&mut v as *mut i32),
        (&mut v as *mut i32),
        (&mut v as *mut i32),
    ];
    let mut e: *mut *mut i32 = (&mut ptrs[(2) as usize] as *mut *mut i32);
    let mut f: *mut *mut i32 = (((ptrs.as_mut_ptr() as *mut libc::c_char)
        .offset(((2_usize).wrapping_mul((::std::mem::size_of::<*mut i32>() as usize))) as isize))
        as *mut *mut i32);
    assert!(((e) == (f)));
    assert!(((&mut ptrs[(1) as usize] as *mut *mut i32) < (f)));
    assert!(((f) < (&mut ptrs[(3) as usize] as *mut *mut i32)));
    assert!(
        ((((f as usize - ptrs.as_mut_ptr() as usize) / ::std::mem::size_of::<*mut i32>()) as i64)
            == (2_i64))
    );
    let mut s: P = <P>::default();
    let mut a: *mut *mut i32 = (&mut s.a as *mut *mut i32);
    let mut b: *mut *mut i32 = (&mut s.arr[(1) as usize] as *mut *mut i32);
    assert!(((a) < (b)));
    assert!(
        (((((&mut s as *mut P) as *mut libc::c_char).offset(
            ((2_usize).wrapping_mul((::std::mem::size_of::<*mut i32>() as usize))) as isize
        )) as *mut *mut i32)
            == (b))
    );
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut i1: i32 = 42;
    let mut ptr1: *mut i32 = (&mut i1 as *mut i32);
    let mut ptr2: *mut libc::c_char = ((&mut i1 as *mut i32) as *mut libc::c_char);
    let mut vptr1: *mut ::libc::c_void = (ptr1 as *mut ::libc::c_void);
    let mut vptr2: *mut ::libc::c_void = (ptr2 as *mut ::libc::c_void);
    assert!(((vptr1) == (vptr2)));
    (unsafe { typed_0() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
