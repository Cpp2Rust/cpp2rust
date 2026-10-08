extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct F {
    pub n: i32,
    pub tail: [libc::c_char; 0],
}
impl Default for F {
    fn default() -> Self {
        F { n: 0_i32, tail: [] }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((::std::mem::size_of::<F>()) == (::std::mem::size_of::<i32>())) as i32) != 0));
    let mut f: *mut F =
        (libcc2rs::malloc_unsafe((::std::mem::size_of::<F>() as usize).wrapping_add(4_usize))
            as *mut F);
    assert!((((!((f).is_null())) as i32) != 0));
    (*f).n = 4;
    {
        if 4_usize != 0 {
            ::std::ptr::copy_nonoverlapping(
                (c"xyz".as_ptr().cast_mut() as *const ::libc::c_void),
                ((*f).tail.as_mut_ptr() as *mut ::libc::c_void),
                4_usize as usize,
            )
        }
        ((*f).tail.as_mut_ptr() as *mut ::libc::c_void)
    };
    assert!((((((*f).n) == (4)) as i32) != 0));
    assert!(
        ((((libc::strcmp(
            ((*f).tail.as_mut_ptr()).cast_const(),
            (c"xyz".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!((((((*(*f).tail.as_mut_ptr().add((1) as usize)) as i32) == ('y' as i32)) as i32) != 0));
    libcc2rs::free_unsafe((f as *mut ::libc::c_void));
    let mut g: *mut F =
        (libcc2rs::malloc_unsafe((::std::mem::size_of::<F>() as usize).wrapping_add(8_usize))
            as *mut F);
    assert!((((!((g).is_null())) as i32) != 0));
    (*g).n = 8;
    {
        if 8_usize != 0 {
            ::std::ptr::copy_nonoverlapping(
                (c"abcdefg".as_ptr().cast_mut() as *const ::libc::c_void),
                ((*g).tail.as_mut_ptr() as *mut ::libc::c_void),
                8_usize as usize,
            )
        }
        ((*g).tail.as_mut_ptr() as *mut ::libc::c_void)
    };
    assert!((((((*(*g).tail.as_mut_ptr().add((6) as usize)) as i32) == ('g' as i32)) as i32) != 0));
    assert!(
        (((((*(*g).tail.as_mut_ptr().add((7) as usize)) as i32) == ('\0' as i32)) as i32) != 0)
    );
    libcc2rs::free_unsafe((g as *mut ::libc::c_void));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
