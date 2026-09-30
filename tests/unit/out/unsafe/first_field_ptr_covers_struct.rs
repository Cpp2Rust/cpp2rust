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
pub struct In {
    pub a: i16,
    pub b: i16,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, Default)]
pub struct S {
    pub x: i32,
    pub in_: In,
    pub z: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut src: S = S {
        x: 1,
        in_: In { a: 2_i16, b: 3_i16 },
        z: 4,
    };
    let mut p: *mut S = (libcc2rs::malloc_unsafe(::std::mem::size_of::<S>()) as *mut S);
    assert!((((!((p).is_null())) as i32) != 0));
    {
        if ::std::mem::size_of::<S>() != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((&mut src as *mut S) as *const ::libc::c_void),
                ((&mut (*p).x as *mut i32) as *mut ::libc::c_void),
                ::std::mem::size_of::<S>() as usize,
            )
        }
        ((&mut (*p).x as *mut i32) as *mut ::libc::c_void)
    };
    assert!(
        ((((((((((((((*p).x) == (1)) as i32) != 0) && (((((*p).in_.a as i32) == (2)) as i32) != 0))
            as i32)
            != 0)
            && (((((*p).in_.b as i32) == (3)) as i32) != 0)) as i32)
            != 0)
            && (((((*p).z) == (4)) as i32) != 0)) as i32)
            != 0)
    );
    let mut n: In = In { a: 5_i16, b: 6_i16 };
    {
        if ::std::mem::size_of::<In>() != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((&mut n as *mut In) as *const ::libc::c_void),
                ((&mut (*p).in_ as *mut In) as *mut ::libc::c_void),
                ::std::mem::size_of::<In>() as usize,
            )
        }
        ((&mut (*p).in_ as *mut In) as *mut ::libc::c_void)
    };
    assert!(
        ((((((((((((((*p).x) == (1)) as i32) != 0) && (((((*p).in_.a as i32) == (5)) as i32) != 0))
            as i32)
            != 0)
            && (((((*p).in_.b as i32) == (6)) as i32) != 0)) as i32)
            != 0)
            && (((((*p).z) == (4)) as i32) != 0)) as i32)
            != 0)
    );
    let mut bz: *mut u8 = ((&mut (*p).z as *mut i32) as *mut u8);
    let mut i: i32 = 0;
    'loop_: while ((((i) < (4)) as i32) != 0) {
        (*bz.offset((i) as isize)) = 1_u8;
        i.postfix_inc();
    }
    assert!(
        ((((((((*p).z) == (16843009)) as i32) != 0) && (((((*p).in_.b as i32) == (6)) as i32) != 0))
            as i32)
            != 0)
    );
    libcc2rs::free_unsafe((p as *mut ::libc::c_void));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
