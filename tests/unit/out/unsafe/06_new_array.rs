extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn take_0(p: *mut *mut libc::c_char) -> libc::c_char {
    (*(*p).offset((0) as isize)) = (9 as libc::c_char);
    let mut c: libc::c_char = (*(*p).offset((0) as isize));
    {
        let __p = (*p);
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<libc::c_char>(),
            )))
        }
    };
    return c;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut e: *mut i32 =
        Box::leak((0..2_usize).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
    (*e.offset((0) as isize)) = 6;
    (*e.offset((1) as isize)) = 7;
    {
        let __p = e;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<i32>(),
            )))
        }
    };
    let mut c: libc::c_char = (unsafe {
        let mut _p: *mut libc::c_char = Box::leak(
            (0..4_usize)
                .map(|_| (0 as libc::c_char))
                .collect::<Box<[libc::c_char]>>(),
        )
        .as_mut_ptr();
        take_0(&mut _p)
    });
    &(c);
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
