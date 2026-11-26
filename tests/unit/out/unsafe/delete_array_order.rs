extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Element {
    pub id: i64,
    pub value: i64,
}
impl Element {
    pub unsafe fn destructor(&mut self) {
        (unsafe {
            libc::printf(
                c"%ld:%ld ".as_ptr() as *const libc::c_char,
                (self.id),
                (self.value),
            )
        });
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: *mut Element = Box::leak(Box::new([
        Element {
            id: 1_i64,
            value: 10_i64,
        },
        Element {
            id: 2_i64,
            value: 20_i64,
        },
        Element {
            id: 3_i64,
            value: 30_i64,
        },
    ]))
    .as_mut_ptr();
    {
        let __p = a;
        if !__p.is_null() {
            for __i in 0..libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                / ::std::mem::size_of::<Element>()
            {
                Element::destructor(&mut *__p.add(__i));
            }
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<Element>(),
            )))
        }
    };
    (unsafe { libc::printf(c"\n".as_ptr() as *const libc::c_char) });
    let mut n: i32 = 1;
    let mut b: *mut Element = Box::leak(
        (0..(n as usize))
            .map(|_| <Element>::default())
            .collect::<Box<[Element]>>(),
    )
    .as_mut_ptr();
    (*b.offset((0) as isize)).id = 4_i64;
    (*b.offset((0) as isize)).value = 40_i64;
    {
        let __p = b;
        if !__p.is_null() {
            for __i in 0..libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                / ::std::mem::size_of::<Element>()
            {
                Element::destructor(&mut *__p.add(__i));
            }
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<Element>(),
            )))
        }
    };
    (unsafe { libc::printf(c"\n".as_ptr() as *const libc::c_char) });
    let mut empty: *mut Element = std::ptr::null_mut();
    {
        let __p = empty;
        if !__p.is_null() {
            for __i in 0..libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                / ::std::mem::size_of::<Element>()
            {
                Element::destructor(&mut *__p.add(__i));
            }
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<Element>(),
            )))
        }
    };
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
