extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, Default)]
pub struct S {
    pub n: i32,
    pub name: [libc::c_char; 1],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, Default)]
pub struct E {
    pub id: i32,
    pub w: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, Default)]
pub struct T {
    pub n: i32,
    pub cap: i32,
    pub a: [E; 1],
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: *mut S = (libcc2rs::calloc_unsafe(
        1_usize,
        (::std::mem::size_of::<S>() as usize).wrapping_add(8_usize),
    ) as *mut S);
    assert!((((!((s).is_null())) as i32) != 0));
    {
        if 8_usize != 0 {
            ::std::ptr::copy_nonoverlapping(
                (c"abcdefg".as_ptr().cast_mut() as *const ::libc::c_void),
                ((*s).name.as_mut_ptr() as *mut ::libc::c_void),
                8_usize as usize,
            )
        }
        ((*s).name.as_mut_ptr() as *mut ::libc::c_void)
    };
    (*s).n = 5;
    assert!((((((*s).n) == (5)) as i32) != 0));
    assert!(
        ((((libc::strcmp(
            ((*s).name.as_mut_ptr()).cast_const(),
            (c"abcdefg".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    libcc2rs::free_unsafe((s as *mut ::libc::c_void));
    let mut t: *mut T = (libcc2rs::malloc_unsafe(
        (::std::mem::size_of::<T>() as usize).wrapping_add((::std::mem::size_of::<E>() as usize)),
    ) as *mut T);
    assert!((((!((t).is_null())) as i32) != 0));
    (*t).n = 2;
    (*t).cap = 2;
    (*(*t).a.as_mut_ptr().add((0) as usize)).id = 10;
    (*(*t).a.as_mut_ptr().add((1) as usize)).w = 20;
    (*t).n = 3;
    assert!(
        ((((((((*(*t).a.as_mut_ptr().add((0) as usize)).id) == (10)) as i32) != 0)
            && (((((*(*t).a.as_mut_ptr().add((1) as usize)).w) == (20)) as i32) != 0))
            as i32)
            != 0)
    );
    let mut tail: *mut E = ((&mut (*t.offset((1) as isize)) as *mut T) as *mut T as *mut E);
    assert!(((((tail) == ((*t).a.as_mut_ptr().add((1) as usize))) as i32) != 0));
    (*tail.offset((0) as isize)).id = 30;
    (*t).cap = 4;
    assert!(
        ((((((((*(*t).a.as_mut_ptr().add((1) as usize)).id) == (30)) as i32) != 0)
            && (((((*(*t).a.as_mut_ptr().add((1) as usize)).w) == (20)) as i32) != 0))
            as i32)
            != 0)
    );
    assert!(
        ((((((((*t).n) == (3)) as i32) != 0) && (((((*t).cap) == (4)) as i32) != 0)) as i32) != 0)
    );
    libcc2rs::free_unsafe((t as *mut ::libc::c_void));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
