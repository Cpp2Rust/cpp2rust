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
pub struct Pointers {
    pub x1: *mut i32,
    pub x2: *const i32,
    pub x3: [*mut i32; 5],
    pub x4: [*const i32; 10],
    pub x5: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SmallArrays {
    pub a: [i32; 32],
    pub f: Option<unsafe fn(i32) -> i32>,
    pub fs: [Option<unsafe fn(i32) -> i32>; 2],
    pub p: [Pointers; 2],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct BigArray {
    pub a: [i32; 33],
}
impl Default for BigArray {
    fn default() -> Self {
        BigArray { a: [0_i32; 33] }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut default_pointers: *mut Pointers = Box::leak(
        (0..10_usize)
            .map(|_| <Pointers>::default())
            .collect::<Box<[Pointers]>>(),
    )
    .as_mut_ptr();
    {
        let __p = default_pointers;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<Pointers>(),
            )))
        }
    };
    let mut small: *mut SmallArrays = Box::leak(
        (0..2_usize)
            .map(|_| <SmallArrays>::default())
            .collect::<Box<[SmallArrays]>>(),
    )
    .as_mut_ptr();
    {
        let __p = small;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<SmallArrays>(),
            )))
        }
    };
    let mut big: *mut BigArray = Box::leak(
        (0..2_usize)
            .map(|_| <BigArray>::default())
            .collect::<Box<[BigArray]>>(),
    )
    .as_mut_ptr();
    {
        let __p = big;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<BigArray>(),
            )))
        }
    };
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
