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
    pub before: i32,
    pub mask: [u8; 4],
    pub after: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: *mut S = (libcc2rs::malloc_unsafe(::std::mem::size_of::<S>()) as *mut S);
    assert!((((!((s).is_null())) as i32) != 0));
    (*s).before = 1;
    {
        let byte_0 = ((*s).mask.as_mut_ptr() as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<[u8; 4]>() {
            *byte_0.offset(offset as isize) = 5 as u8;
        }
        ((*s).mask.as_mut_ptr() as *mut ::libc::c_void)
    };
    (*s).after = 2;
    (*((&mut (*s).mask as *mut [u8; 4]) as *mut u8)) = 7_u8;
    let mut out: [u8; 4] = [0_u8; 4];
    {
        if ::std::mem::size_of::<[u8; 4]>() != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((&mut (*s).mask as *mut [u8; 4]) as *const ::libc::c_void),
                (out.as_mut_ptr() as *mut ::libc::c_void),
                ::std::mem::size_of::<[u8; 4]>() as usize,
            )
        }
        (out.as_mut_ptr() as *mut ::libc::c_void)
    };
    assert!(
        (((((((out[(0) as usize] as i32) == (7)) as i32) != 0)
            && ((((out[(3) as usize] as i32) == (5)) as i32) != 0)) as i32)
            != 0)
    );
    assert!(
        ((((((((*s).before) == (1)) as i32) != 0) && (((((*s).after) == (2)) as i32) != 0))
            as i32)
            != 0)
    );
    libcc2rs::free_unsafe((s as *mut ::libc::c_void));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
