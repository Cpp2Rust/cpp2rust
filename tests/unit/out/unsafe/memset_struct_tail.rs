extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg)]
pub struct S {
    pub keep: i32,
    pub a: i32,
    pub b: i64,
    pub c: [libc::c_char; 5],
    pub last: i32,
}
impl Default for S {
    fn default() -> Self {
        S {
            keep: 0_i32,
            a: 0_i32,
            b: 0_i64,
            c: [(0 as libc::c_char); 5],
            last: 0_i32,
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: *mut S = (libcc2rs::malloc_unsafe(::std::mem::size_of::<S>()) as *mut S);
    assert!((((!((p).is_null())) as i32) != 0));
    {
        let byte_0 = (p as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<S>() {
            *byte_0.offset(offset as isize) = 255 as u8;
        }
        (p as *mut ::libc::c_void)
    };
    (*p).keep = 7;
    {
        let byte_0 = ((&mut (*p).a as *mut i32) as *mut ::libc::c_void) as *mut u8;
        for offset in 0..(::std::mem::size_of::<S>() as usize)
            .wrapping_sub((::std::mem::offset_of!(S, a) as usize))
        {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        ((&mut (*p).a as *mut i32) as *mut ::libc::c_void)
    };
    assert!((((((*p).keep) == (7)) as i32) != 0));
    assert!(
        ((((((((((((((*p).a) == (0)) as i32) != 0) && (((((*p).b) == (0_i64)) as i32) != 0))
            as i32)
            != 0)
            && (((((*p).c[(4) as usize] as i32) == (0)) as i32) != 0)) as i32)
            != 0)
            && (((((*p).last) == (0)) as i32) != 0)) as i32)
            != 0)
    );
    (*p).a = 1;
    (*p).b = 2_i64;
    (*p).c[(0) as usize] = (('x' as i32) as libc::c_char);
    (*p).last = 3;
    {
        let byte_0 = ((&mut (*p).b as *mut i64) as *mut ::libc::c_void) as *mut u8;
        for offset in 0..(::std::mem::offset_of!(S, last) as usize)
            .wrapping_sub((::std::mem::offset_of!(S, b) as usize))
        {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        ((&mut (*p).b as *mut i64) as *mut ::libc::c_void)
    };
    assert!(
        ((((((((*p).keep) == (7)) as i32) != 0) && (((((*p).a) == (1)) as i32) != 0)) as i32) != 0)
    );
    assert!(
        ((((((((*p).b) == (0_i64)) as i32) != 0)
            && (((((*p).c[(0) as usize] as i32) == (0)) as i32) != 0)) as i32)
            != 0)
    );
    assert!((((((*p).last) == (3)) as i32) != 0));
    libcc2rs::free_unsafe((p as *mut ::libc::c_void));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
